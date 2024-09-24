use kube::config::Kubeconfig;

use crate::configuration::KubeConfiguration;
use crate::fonts;
use crate::segments::{Segment, SegmentGenerator};
use crate::theme::{BackgroundColor, ForegroundColor, Theme};
use crate::Shell;

pub struct KubeSegment<'a> {
    config: Option<&'a KubeConfiguration>,
}

impl<'a> KubeSegment<'a> {
    pub fn new(config: Option<&'a KubeConfiguration>) -> Self {
        Self { config }
    }
}

impl<'a> SegmentGenerator for KubeSegment<'a> {
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

        if self.config.is_some_and(|c| {
            c.critical_contexts
                .as_deref()
                .unwrap_or_default()
                .iter()
                .any(|c| current_context.contains(c))
        }) {
            segments.push(Segment {
                text: format!(r"{}", fonts::NerdFonts::FA_WARNING),
                fg: ForegroundColor(196),
                bg,
                blinking: true,
            })
        }

        let alias = self.config.and_then(|c| {
            c.context_aliases
                .as_deref()
                .unwrap_or_default()
                .iter()
                .find(|ka| ka.context == current_context)
        });
        segments.push(Segment {
            text: format!(
                "{} ",
                alias
                    .map(|a| a.alias.as_str())
                    .unwrap_or(current_context.as_str())
            ),
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
