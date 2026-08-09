use kube::config::Kubeconfig;

use crate::Shell;
use crate::configuration::KubeConfiguration;
use crate::segments::{Segment, SegmentSection};
use crate::theme::{ForegroundColor, Theme};

pub struct KubeSegment<'a> {
    config: Option<&'a KubeConfiguration>,
}

impl<'a> KubeSegment<'a> {
    pub fn new(config: Option<&'a KubeConfiguration>) -> Self {
        Self { config }
    }
}

impl Segment for KubeSegment<'_> {
    fn name(&self) -> &'static str {
        "kube"
    }

    fn output(&self, _shell: Shell, theme: &Theme) -> Option<Vec<SegmentSection>> {
        let config = Kubeconfig::read().ok()?;
        let current_context = config.current_context?;
        let context = config
            .contexts
            .iter()
            .find(|c| c.name == current_context)
            .map(|c| c.context.as_ref())??;
        let mut sections = Vec::new();

        sections.push(SegmentSection::Section {
            text: "⎈".into(),
            bg: theme.kube_context_bg,
            fg: theme.kube_context_fg,
            blinking: false,
        });

        if let Some(config) = self.config
            && let Some(critical_contexts) = &config.critical_contexts
            && critical_contexts
                .iter()
                .any(|c| current_context.contains(c))
        {
            sections.push(SegmentSection::Section {
                text: "".into(),
                bg: theme.kube_context_bg,
                fg: ForegroundColor::from_color_code(196),
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
        sections.push(SegmentSection::Section {
            text: alias
                .map(|a| a.alias.as_str())
                .unwrap_or(current_context.as_str())
                .to_string()
                .into(),
            bg: theme.kube_context_bg,
            fg: theme.kube_context_fg,
            blinking: false,
        });

        if let Some(ref namespace) = context.namespace {
            sections.push(SegmentSection::Section {
                text: namespace.to_string().into(),
                bg: theme.kube_namespace_bg,
                fg: theme.kube_namespace_fg,
                blinking: false,
            })
        }

        Some(sections)
    }
}
