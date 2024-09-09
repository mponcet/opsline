use kube::config::Kubeconfig;

use crate::{
    segments::SegmentOutput,
    theme::{BackgroundColor, ForegroundColor, Theme},
    Segment, Shell,
};

pub struct KubeSegment;

impl KubeSegment {
    pub fn new() -> Self {
        Self {}
    }
}

impl Segment for KubeSegment {
    fn output(&self, _shell: Shell, theme: &Theme) -> SegmentOutput {
        let current_context = match Kubeconfig::read() {
            Ok(config) => {
                if let Some(current_context) = config.current_context {
                    if config.contexts.iter().any(|c| c.name == current_context) {
                        current_context
                    } else {
                        "".into()
                    }
                } else {
                    "".into()
                }
            }
            Err(_) => "".into(),
        };

        let (fg, bg) = match theme {
            Theme::Default => (ForegroundColor(117), BackgroundColor(26)),
        };

        SegmentOutput {
            text: current_context,
            fg,
            bg,
        }
    }
}
