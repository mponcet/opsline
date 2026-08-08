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
    fn name(&self) -> &'static str {
        "root"
    }

    fn output(&self, shell: Shell, theme: &Theme) -> Option<Vec<Segment>> {
        let text = match shell {
            Shell::Bash => r" \$ ",
            Shell::Zsh => " %# ",
            Shell::Fish => {
                let is_root = unsafe { libc::geteuid() } == 0;
                if is_root { " # " } else { " $ " }
            }
        };

        Some(Vec::from([Segment {
            name: "root",
            text: text.into(),
            bg: theme.root_bg,
            fg: theme.root_fg,
            blinking: false,
        }]))
    }
}
