use crate::configuration::ContainersConfiguration;
use crate::fonts;
use crate::segments::{Segment, SegmentGenerator};
use crate::shell::Shell;
use crate::theme::{BackgroundColor, ForegroundColor, Theme};
use crate::utils::ureq_unix::{FakeResolver, UnixConnector};
use ureq::{Agent, Config};

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

fn list_containers(url: &str) -> Option<Vec<Container>> {
    log::info!("listing containers at {}", url);
    let request = if let Some(path) = url.strip_prefix("unix:") {
        let config = Config::default();
        let resolver = FakeResolver;
        let connector = UnixConnector::new(path);
        let agent = Agent::with_parts(config, connector, resolver);

        agent.get("http://d/containers/json?all=true")
    } else {
        ureq::get(format!("{}/containers/json?all=true", url))
    };

    let mut response = request
        .call()
        .map_err(|_| {
            log::error!("http request failed");
        })
        .ok()?;

    response
        .body_mut()
        .read_json::<Vec<Container>>()
        .map_err(|_| {
            log::error!("body deserialization failed");
        })
        .ok()
}

impl<'a> SegmentGenerator for ContainersSegment<'a> {
    fn output(&self, _shell: Shell, theme: Theme) -> Option<Vec<Segment>> {
        let url = self.config.as_ref()?.url.as_ref();
        let containers = list_containers(url)?;

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
