use kube::config::Kubeconfig;

use crate::{
    fonts,
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

        Some(SegmentOutput { text, fg, bg })
    }
}
