use crate::configuration::TerraformConfiguration;
use crate::segments::{Segment, SegmentSection};
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

impl Segment for TerraformSegment<'_> {
    fn name(&self) -> &'static str {
        "terraform"
    }

    fn output(&self, _shell: Shell, theme: &Theme) -> Option<Vec<SegmentSection>> {
        let workspace = std::fs::read_to_string(".terraform/environment").ok()?;
        let mut sections = Vec::new();

        sections.push(SegmentSection::Section {
            text: " 󱁢 ".into(),
            bg: theme.terraform_bg,
            fg: theme.terraform_fg,
            blinking: false,
        });

        if let Some(config) = self.config
            && config.critical_workspaces.contains(&workspace)
        {
            sections.push(SegmentSection::Section {
                text: "".into(),
                bg: theme.terraform_bg,
                fg: ForegroundColor::from_color_code(196),
                blinking: true,
            });
        }

        sections.push(SegmentSection::Section {
            text: format!("{workspace} ").into(),
            bg: theme.terraform_bg,
            fg: theme.terraform_fg,
            blinking: false,
        });

        Some(sections)
    }
}
