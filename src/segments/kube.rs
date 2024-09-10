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
    fn output(&self, _shell: Shell, theme: &Theme) -> Option<SegmentOutput> {
        let config = Kubeconfig::read().ok()?;
        let current_context = config.current_context?;

        if config.contexts.iter().all(|c| c.name != current_context) {
            return None;
        }

        let (fg, bg) = match theme {
            Theme::Default => (ForegroundColor(117), BackgroundColor(26)),
        };

        Some(SegmentOutput {
            text: current_context,
            fg,
            bg,
        })
    }
}
