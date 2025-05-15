use crate::Shell;
use crate::Theme;
use crate::{SegmentGenerator, segments::Segment};

pub struct RootSegment;

impl RootSegment {
    pub fn new() -> Self {
        Self {}
    }
}

impl SegmentGenerator for RootSegment {
    fn output(&self, shell: Shell, theme: &Theme) -> Option<Vec<Segment>> {
        let text = match shell {
            Shell::Bash => r" \$ ",
            Shell::Zsh => " %# ",
        };

        Some(Vec::from([Segment {
            text: text.into(),
            bg: theme.root_bg,
            fg: theme.root_fg,
            blinking: false,
        }]))
    }
}
