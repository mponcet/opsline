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

impl SegmentGenerator for TerraformSegment<'_> {
    fn output(&self, _shell: Shell, theme: &Theme) -> Option<Vec<Segment>> {
        let workspace = std::fs::read_to_string(".terraform/environment").ok()?;
        let mut segments = Vec::new();

        segments.push(Segment {
            name: "terraform",
            text: " 󱁢 ".into(),
            bg: theme.terraform_bg,
            fg: theme.terraform_fg,
            blinking: false,
        });

        if let Some(config) = self.config
            && config.critical_workspaces.contains(&workspace)
        {
            segments.push(Segment {
                name: "terraform",
                text: "".into(),
                bg: theme.terraform_bg,
                fg: ForegroundColor::from_color_code(196),
                blinking: true,
            });
        }

        segments.push(Segment {
            name: "terraform",
            text: format!("{workspace} ").into(),
            bg: theme.terraform_bg,
            fg: theme.terraform_fg,
            blinking: false,
        });

        Some(segments)
    }
}
