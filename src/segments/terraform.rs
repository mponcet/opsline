use crate::configuration::TerraformConfiguration;
use crate::segments::{Segment, SegmentGenerator};
use crate::shell::Shell;
use crate::theme::{ForegroundColor, Theme};

pub struct TerraformSegment<'a> {
    config: Option<&'a TerraformConfiguration>,
}

impl<'a> TerraformSegment<'a> {
    pub fn new(config: Option<&'a TerraformConfiguration>) -> Self {
        Self { config }
    }
}

impl<'a> SegmentGenerator for TerraformSegment<'a> {
    fn output(&self, _shell: Shell, theme: &Theme) -> Option<Vec<Segment>> {
        let workspace = std::fs::read_to_string(".terraform/environment").ok()?;
        let mut segments = Vec::new();

        segments.push(Segment {
            text: " 󱁢 ".into(),
            bg: theme.terraform_bg,
            fg: theme.terraform_fg,
            blinking: false,
        });

        if self
            .config
            .is_some_and(|c| c.critical_workspaces.iter().any(|w| *w == workspace))
        {
            segments.push(Segment {
                text: "".into(),
                bg: theme.terraform_bg,
                fg: ForegroundColor(196),
                blinking: true,
            });
        }

        segments.push(Segment {
            text: format!("{workspace} ").into(),
            bg: theme.terraform_bg,
            fg: theme.terraform_fg,
            blinking: false,
        });

        Some(segments)
    }
}
