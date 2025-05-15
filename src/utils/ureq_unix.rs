use std::fmt::Debug;
use std::io::{self, Read, Write};
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::os::unix::net::UnixStream;
use std::path::{Path, PathBuf};

use ureq::config::Config;
use ureq::http::Uri;
use ureq::unversioned::resolver::{ArrayVec, ResolvedSocketAddrs, Resolver};
use ureq::unversioned::transport::{
    Buffers, ConnectionDetails, Connector, LazyBuffers, NextTimeout, Transport,
};

#[derive(Debug)]
pub struct UnixConnector {
    path: PathBuf,
}

impl UnixConnector {
    pub fn new<P: AsRef<Path>>(path: P) -> Self {
        Self {
            path: path.as_ref().into(),
        }
    }
}

impl Connector for UnixConnector {
    type Out = Box<dyn Transport>;

    fn connect(
        &self,
        details: &ConnectionDetails,
        chained: Option<()>,
    ) -> Result<Option<Box<dyn Transport>>, ureq::Error> {
        if chained.is_some() {
            // do something ?
        }

        let config = details.config;
        let stream = UnixStream::connect(self.path.as_path()).map_err(|e| {
            log::error!("connection failed: {}", e);
            ureq::Error::Io(e)
        })?;

        let buffers = LazyBuffers::new(config.input_buffer_size(), config.output_buffer_size());
        let transport = UnixStreamTransport::new(stream, buffers);

        Ok(Some(Box::new(transport)))
    }
}

struct UnixStreamTransport {
    stream: UnixStream,
    buffers: LazyBuffers,
}

impl std::fmt::Debug for UnixStreamTransport {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("UnixStreamTransport")
            .field("stream", &self.stream)
            .finish()
    }
}

impl UnixStreamTransport {
    fn new(stream: UnixStream, buffers: LazyBuffers) -> Self {
        UnixStreamTransport { stream, buffers }
    }
}

impl Transport for UnixStreamTransport {
    fn buffers(&mut self) -> &mut dyn Buffers {
        &mut self.buffers
    }

    fn transmit_output(&mut self, amount: usize, timeout: NextTimeout) -> Result<(), ureq::Error> {
        let output = &self.buffers.output()[..amount];
        self.stream.set_write_timeout(Some(*timeout.after))?;
        self.stream.write_all(output).map_err(|e| {
            log::error!("{:?}", e);
            e
        })?;

        Ok(())
    }

    fn await_input(&mut self, timeout: NextTimeout) -> Result<bool, ureq::Error> {
        if self.buffers.can_use_input() {
            return Ok(true);
        }

        let input = self.buffers.input_append_buf();
        self.stream.set_read_timeout(Some(*timeout.after))?;
        let amount = self.stream.read(input).map_err(|e| {
            log::error!("{:?}", e);
            e
        })?;
        self.buffers.input_appended(amount);

        Ok(amount > 0)
    }

    fn is_open(&mut self) -> bool {
        let mut buf = [0];

        if self.stream.set_nonblocking(true).is_err() {
            return false;
        }

        let ret = match self.stream.read(&mut buf) {
            Ok(_) => true,
            Err(e) => e.kind() == io::ErrorKind::WouldBlock,
        };

        if self.stream.set_nonblocking(false).is_err() {
            return false;
        }

        ret
    }
}

// with unix socket transport, dns resolution is not needed
// return an empty vec of SocketAddrs
#[derive(Debug, Default)]
pub struct FakeResolver;

impl Resolver for FakeResolver {
    fn resolve(
        &self,
        _uri: &Uri,
        _config: &Config,
        _timeout: NextTimeout,
    ) -> Result<ResolvedSocketAddrs, ureq::Error> {
        fn localhost_socketaddr() -> SocketAddr {
            SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 0)
        }

        Ok(ArrayVec::from_fn(|_| localhost_socketaddr()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;
    use ureq::Agent;

    #[test]
    fn test_podman_unix_socket() {
        std::process::Command::new("systemctl")
            .args(["start", "--user", "podman.socket"])
            .output()
            .expect("failed to start podman socket");
        let config = Config::default();
        let resolver = FakeResolver;
        let connector = UnixConnector::new(format!("/run/user/{}/podman/podman.sock", unsafe {
            libc::getuid()
        }));
        let agent = ureq::Agent::with_parts(config, connector, resolver);

        let response = agent.get("http://d/_ping").call();
        match response {
            Ok(mut response) => assert_eq!(response.body_mut().read_to_string().unwrap(), "OK"),
            Err(_) => panic!("failed to ping podman"),
        }
    }

    #[test]
    #[should_panic]
    fn test_podman_unix_socket_timeout() {
        let config = Agent::config_builder()
            .timeout_global(Some(Duration::from_millis(1)))
            .build();
        let resolver = FakeResolver;
        let connector = UnixConnector::new(format!("/run/user/{}/podman/podman.sock", unsafe {
            libc::getuid()
        }));
        let agent = ureq::Agent::with_parts(config, connector, resolver);

        let response = agent.get("http://d/_ping").call();
        match response {
            Ok(mut response) => assert_eq!(response.body_mut().read_to_string().unwrap(), "OK"),
            Err(_) => panic!("failed to ping podman"),
        }
    }
}
