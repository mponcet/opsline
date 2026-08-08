use crate::Shell;
use crate::Theme;
use crate::{Segment, segments::SegmentSection};

pub struct RootSegment;

impl RootSegment {
    pub fn new() -> Self {
        Self {}
    }
}

impl Segment for RootSegment {
    fn name(&self) -> &'static str {
        "root"
    }

    fn output(&self, shell: Shell, theme: &Theme) -> Option<Vec<SegmentSection>> {
        let text = match shell {
            Shell::Bash => r" \$ ",
            Shell::Zsh => " %# ",
        };

        Some(Vec::from([SegmentSection {
            name: "root",
            text: text.into(),
            bg: theme.root_bg,
            fg: theme.root_fg,
            blinking: false,
        }]))
    }
}
