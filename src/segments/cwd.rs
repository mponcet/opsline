use crate::configuration::CwdConfiguration;
use crate::segments::{Segment, SegmentGenerator};
use crate::theme::Theme;
use crate::Shell;

pub struct CwdSegment<'a> {
    config: &'a CwdConfiguration,
}

impl<'a> CwdSegment<'a> {
    pub fn new(config: &'a CwdConfiguration) -> Self {
        Self { config }
    }
}

impl<'a> SegmentGenerator for CwdSegment<'a> {
    fn output(&self, shell: Shell, theme: &Theme) -> Option<Vec<Segment>> {
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

        Some(Vec::from([Segment {
            text: text.into(),
            bg: theme.cwd_bg,
            fg: theme.cwd_fg,
            blinking: false,
        }]))
    }
}
