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

        let text = if let Some(namespace) = context.namespace {
            format!(
                "{} {} | {}",
                fonts::NerdFonts::SHIP_WHEEL,
                current_context,
                namespace
            )
        } else {
            format!("{} {}", fonts::NerdFonts::SHIP_WHEEL, current_context)
        };

        Some(Segments::One(Segment { text, fg, bg }))
    }
}
