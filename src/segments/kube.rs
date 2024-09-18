use kube::config::Kubeconfig;

use crate::fonts;
use crate::segments::{Segment, SegmentGenerator};
use crate::theme::{BackgroundColor, ForegroundColor, Theme};
use crate::Shell;

pub struct KubeSegment;

impl KubeSegment {
    pub fn new() -> Self {
        Self {}
    }
}

impl SegmentGenerator for KubeSegment {
    fn output(&self, _shell: Shell, theme: &Theme) -> Option<Vec<Segment>> {
        let config = Kubeconfig::read().ok()?;
        let current_context = config.current_context?;
        let context = config
            .contexts
            .iter()
            .find(|c| c.name == current_context)
            .map(|c| c.context.clone())??;
        let mut segments = Vec::new();

        let (fg, bg) = match theme {
            Theme::Default => (ForegroundColor(117), BackgroundColor(26)),
        };

        segments.push(Segment {
            text: format!(" {}", fonts::NerdFonts::SHIP_WHEEL),
            fg,
            bg,
            blinking: false,
        });

        // TODO: should be a parameter
        if current_context.contains("prod") {
            segments.push(Segment {
                text: format!(r"{}", fonts::NerdFonts::FA_WARNING),
                fg: ForegroundColor(196),
                bg,
                blinking: true,
            })
        }

        segments.push(Segment {
            text: format!("{} ", current_context),
            fg,
            bg,
            blinking: false,
        });

        if let Some(namespace) = context.namespace {
            let (fg, bg) = match theme {
                Theme::Default => (ForegroundColor(170), BackgroundColor(17)),
            };
            segments.push(Segment {
                text: format!(" {} ", namespace),
                fg,
                bg,
                blinking: false,
            })
        }

        Some(segments)
    }
}
