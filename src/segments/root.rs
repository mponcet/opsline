use crate::theme::{BackgroundColor, ForegroundColor};
use crate::Shell;
use crate::Theme;
use crate::{segments::Segment, SegmentGenerator};

pub struct RootSegment;

impl RootSegment {
    pub fn new() -> Self {
        Self {}
    }
}

impl SegmentGenerator for RootSegment {
    fn output(&self, shell: Shell, theme: Theme) -> Option<Vec<Segment>> {
        let text = match shell {
            Shell::Bash => r" \$ ",
            Shell::Zsh => " %# ",
        };

        let (bg, fg) = match theme {
            Theme::Default => (BackgroundColor(236), ForegroundColor(15)),
            Theme::Gruvbox => (BackgroundColor(237), ForegroundColor(246)),
        };

        Some(Vec::from([Segment {
            text: text.into(),
            bg,
            fg,
            blinking: false,
        }]))
    }
}
