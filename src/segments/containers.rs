use std::fmt::Debug;
use std::io::{self, Read, Write};
use std::os::unix::net::UnixStream;
use std::path::{Path, PathBuf};

use ureq::http::Uri;
use ureq::resolver::{ResolvedSocketAddrs, Resolver};
use ureq::transport::{Buffers, ConnectionDetails, Connector, LazyBuffers, NextTimeout, Transport};
use ureq::{Agent, Config};

use crate::configuration::ContainersConfiguration;
use crate::fonts;
use crate::segments::{Segment, SegmentGenerator};
use crate::shell::Shell;
use crate::theme::{BackgroundColor, ForegroundColor, Theme};

pub struct ContainersSegment<'a> {
    config: Option<&'a ContainersConfiguration>,
}

impl<'a> ContainersSegment<'a> {
    pub fn new(config: Option<&'a ContainersConfiguration>) -> Self {
        Self { config }
    }
}

#[derive(Debug)]
struct UnixConnector {
    path: PathBuf,
}

impl UnixConnector {
    fn new<P: AsRef<Path>>(path: P) -> Self {
        Self {
            path: path.as_ref().into(),
        }
    }
}

impl Connector for UnixConnector {
    fn connect(
        &self,
        details: &ConnectionDetails,
        chained: Option<Box<dyn Transport>>,
    ) -> Result<Option<Box<dyn Transport>>, ureq::Error> {
        if chained.is_some() {
            // do something ?
        }

        let config = details.config;
        let stream = UnixStream::connect(self.path.as_path())?;
        let buffers = LazyBuffers::new(config.input_buffer_size, config.output_buffer_size);
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

    fn transmit_output(&mut self, amount: usize, _timeout: NextTimeout) -> Result<(), ureq::Error> {
        let output = &self.buffers.output()[..amount];
        self.stream.write_all(output)?;

        Ok(())
    }

    fn await_input(&mut self, _timeout: NextTimeout) -> Result<bool, ureq::Error> {
        if self.buffers.can_use_input() {
            return Ok(true);
        }

        let input = self.buffers.input_append_buf();
        let amount = self.stream.read(input)?;
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
struct FakeResolver;

impl Resolver for FakeResolver {
    fn resolve(
        &self,
        _uri: &Uri,
        _config: &Config,
        _timeout: NextTimeout,
    ) -> Result<ResolvedSocketAddrs, ureq::Error> {
        Ok(ResolvedSocketAddrs::new())
    }
}

#[derive(serde::Deserialize)]
struct Container {
    #[serde(rename(deserialize = "State"))]
    state: String,
}

fn list_containers(url: &str) -> Result<Vec<Container>, Box<dyn std::error::Error>> {
    if let Some(path) = url.strip_prefix("unix:") {
        let config = Config::default();
        let resolver = FakeResolver;
        let connector = UnixConnector::new(path);
        let agent = Agent::with_parts(config, connector, resolver);

        match agent.get("http://d/containers/json?all=true").call() {
            Ok(mut response) => Ok(response.body_mut().read_json::<Vec<Container>>()?),
            Err(_) => Err("couldn't list containers".into()),
        }
    } else {
        match ureq::get(format!("{}/containers/json?all=true", url)).call() {
            Ok(mut response) => Ok(response.body_mut().read_json::<Vec<Container>>()?),
            Err(_) => Err("couldn't list containers".into()),
        }
    }
}

impl<'a> SegmentGenerator for ContainersSegment<'a> {
    fn output(&self, _shell: Shell, theme: Theme) -> Option<Vec<Segment>> {
        let url = self.config.as_ref()?.url.as_ref()?;
        let containers = list_containers(url).ok()?;

        let (bg, fg) = match theme {
            Theme::Default => (BackgroundColor(55), ForegroundColor(177)),
            Theme::Gruvbox => (BackgroundColor(96), ForegroundColor(229)),
        };

        // status=(created, restarting, running, removing, paused, exited or dead)
        let (mut running, mut paused, mut exited, mut restarting) = (0, 0, 0, 0);
        for container in &containers {
            match container.state.as_str() {
                "running" => running += 1,
                "paused" => paused += 1,
                "exited" => exited += 1,
                "restarting" => restarting += 1,
                _ => {}
            }
        }
        if (running, paused, exited, restarting) == (0, 0, 0, 0) {
            return None;
        }

        let mut text = format!(" {} ", fonts::NerdFonts::MD_DOCKER);
        {
            use std::fmt::Write;
            if running > 0 {
                write!(text, " {} {}", fonts::NerdFonts::BLACK_CIRCLE, running).unwrap();
            }
            if paused > 0 {
                write!(text, " {} {}", fonts::NerdFonts::TILDE, paused).unwrap();
            }
            if exited > 0 {
                write!(
                    text,
                    " {} {}",
                    fonts::NerdFonts::HEAVY_MULTIPLICATION_X,
                    exited
                )
                .unwrap();
            }
            if restarting > 0 {
                write!(
                    text,
                    " {} {}",
                    fonts::NerdFonts::CLOCKWISE_OPEN_CIRCLE_ARROW,
                    restarting
                )
                .unwrap();
            }
            text.push(' ');
        }

        Some(Vec::from([Segment {
            text,
            bg,
            fg,
            blinking: false,
        }]))
    }
}

#[cfg(test)]
mod tests {
    use ureq::{Agent, Config};

    use super::*;

    #[test]
    fn test_podman_unix_socket() {
        let config = Config::default();
        let resolver = FakeResolver;
        let connector = UnixConnector::new(format!("/run/user/{}/podman/podman.sock", unsafe {
            libc::getuid()
        }));
        let agent = Agent::with_parts(config, connector, resolver);

        let url = "http://d/_ping";
        match agent.get(url).call() {
            Ok(mut result) => assert_eq!(result.body_mut().read_to_string().unwrap(), "OK"),
            Err(_) => panic!("failed to get {}", url),
        }
    }
}
