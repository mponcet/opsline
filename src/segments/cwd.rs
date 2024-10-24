use crate::configuration::CwdConfiguration;
use crate::segments::{Segment, SegmentGenerator};
use crate::theme::{BackgroundColor, ForegroundColor};
use crate::Shell;
use crate::Theme;

pub struct CwdSegment<'a> {
    config: &'a CwdConfiguration,
}

impl<'a> CwdSegment<'a> {
    pub fn new(config: &'a CwdConfiguration) -> Self {
        Self { config }
    }
}

impl<'a> SegmentGenerator for CwdSegment<'a> {
    fn output(&self, shell: Shell, theme: Theme) -> Option<Vec<Segment>> {
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

        let (bg, fg) = match theme {
            Theme::Default => (BackgroundColor(241), ForegroundColor(250)),
            Theme::Gruvbox => (BackgroundColor(66), ForegroundColor(250)),
        };

        Some(Vec::from([Segment {
            text: text.into(),
            bg,
            fg,
            blinking: false,
        }]))
    }
}
