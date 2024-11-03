use std::time::Duration;
use ureq::{Agent, Config, Timeouts};

use crate::configuration::ContainersConfiguration;
use crate::segments::{Segment, SegmentGenerator};
use crate::shell::Shell;
use crate::theme::Theme;
use crate::utils::ureq_unix::{FakeResolver, UnixConnector};

const REQUEST_TIMEOUT_MS: u64 = 500;

pub struct ContainersSegment<'a> {
    config: Option<&'a ContainersConfiguration>,
}

impl<'a> ContainersSegment<'a> {
    pub fn new(config: Option<&'a ContainersConfiguration>) -> Self {
        Self { config }
    }
}

#[derive(serde::Deserialize)]
struct Container {
    #[serde(rename(deserialize = "State"))]
    state: String,
}

fn list_containers<T: AsRef<str>>(url: T, timeout: Option<Duration>) -> Option<Vec<Container>> {
    let url = url.as_ref();
    log::info!("listing containers at {}", url);

    let config = if timeout.is_some() {
        Config {
            timeouts: Timeouts {
                global: timeout,
                ..Default::default()
            },
            ..Default::default()
        }
    } else {
        Config::default()
    };

    let request = if let Some(path) = url.strip_prefix("unix:") {
        let resolver = FakeResolver;
        let connector = UnixConnector::new(path);
        let agent = Agent::with_parts(config, connector, resolver);
        agent.get("http://d/containers/json?all=true")
    } else {
        let agent: Agent = config.into();
        agent.get(format!("{}/containers/json?all=true", url))
    };

    request
        .call()
        .map_err(|_| {
            log::error!("http request failed");
        })
        .ok()?
        .body_mut()
        .read_json::<Vec<Container>>()
        .map_err(|_| {
            log::error!("body deserialization failed");
        })
        .ok()
}

impl<'a> SegmentGenerator for ContainersSegment<'a> {
    fn output(&self, _shell: Shell, theme: &Theme) -> Option<Vec<Segment>> {
        let url = &self.config.as_ref()?.url;
        let containers = list_containers(url, Some(Duration::from_millis(REQUEST_TIMEOUT_MS)))?;

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

        let mut text = " 󰡨 ".to_owned();
        {
            use std::fmt::Write;
            if running > 0 {
                write!(text, " ● {}", running).unwrap();
            }
            if paused > 0 {
                write!(text, " ~ {}", paused).unwrap();
            }
            if exited > 0 {
                write!(text, " ✖ {}", exited).unwrap();
            }
            if restarting > 0 {
                write!(text, " ↻ {}", restarting).unwrap();
            }
            text.push(' ');
        }

        Some(Vec::from([Segment {
            text: text.into(),
            bg: theme.container_bg,
            fg: theme.container_fg,
            blinking: false,
        }]))
    }
}
