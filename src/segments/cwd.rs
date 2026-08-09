use crate::Shell;
use crate::configuration::CwdConfiguration;
use crate::segments::{Segment, SegmentSection};
use crate::theme::Theme;

pub struct CwdSegment<'a> {
    config: &'a CwdConfiguration,
}

impl<'a> CwdSegment<'a> {
    pub fn new(config: &'a CwdConfiguration) -> Self {
        Self { config }
    }
}

impl Segment for CwdSegment<'_> {
    fn name(&self) -> &'static str {
        "cwd"
    }

    fn output(&self, shell: Shell, theme: &Theme) -> Option<Vec<SegmentSection>> {
        let text = if self.config.dironly {
            match shell {
                Shell::Bash => r" \W ",
                Shell::Zsh => " %1d ",
            }
        } else {
            match shell {
                Shell::Bash => r" \w ",
                Shell::Zsh => " %d ",
            }
        };

        Some(Vec::from([SegmentSection::Section {
            text: text.into(),
            bg: theme.cwd_bg,
            fg: theme.cwd_fg,
            blinking: false,
        }]))
    }
}
