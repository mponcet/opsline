use kube::config::Kubeconfig;

use crate::fonts;
use crate::segments::{Segment, SegmentGenerator, Segments};
use crate::theme::{BackgroundColor, ForegroundColor, Theme};
use crate::Shell;

pub struct KubeSegment;

impl KubeSegment {
    pub fn new() -> Self {
        Self {}
    }
}

impl SegmentGenerator for KubeSegment {
    fn output(&self, _shell: Shell, theme: &Theme) -> Option<Segments> {
        let config = Kubeconfig::read().ok()?;
        let current_context = config.current_context?;
        let context = config
            .contexts
            .iter()
            .find(|c| c.name == current_context)
            .map(|c| c.context.clone())??;

        let (fg, bg) = match theme {
            Theme::Default => (ForegroundColor(117), BackgroundColor(26)),
        };

        let segment_context = Segment {
            text: format!(" {} {} ", fonts::NerdFonts::SHIP_WHEEL, current_context),
            fg,
            bg,
            blinking: false,
        };

        // TODO: should be a parameter
        let segment_warning = if current_context.contains("prod") {
            Some(Segment {
                text: format!(r"{} ", fonts::NerdFonts::FA_WARNING),
                fg: ForegroundColor(196),
                bg,
                blinking: true,
            })
        } else {
            None
        };

        let segment_namespace = if let Some(namespace) = context.namespace {
            let (fg, bg) = match theme {
                Theme::Default => (ForegroundColor(170), BackgroundColor(17)),
            };
            Some(Segment {
                text: format!(" {} ", namespace),
                fg,
                bg,
                blinking: false,
            })
        } else {
            None
        };

        match (segment_warning, segment_namespace) {
            (None, None) => Some(Segments::One(segment_context)),
            (None, Some(segment_namespace)) => {
                Some(Segments::Many(vec![segment_context, segment_namespace]))
            }
            (Some(segment_warning), None) => {
                Some(Segments::Many(vec![segment_context, segment_warning]))
            }
            (Some(segment_warning), Some(segment_namespace)) => Some(Segments::Many(vec![
                segment_context,
                segment_warning,
                segment_namespace,
            ])),
        }
    }
}
