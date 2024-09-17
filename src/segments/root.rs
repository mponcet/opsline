use crate::theme::{BackgroundColor, ForegroundColor};
use crate::Shell;
use crate::Theme;
use crate::{segments::Segment, SegmentGenerator, Segments};

pub struct RootSegment;

impl RootSegment {
    pub fn new() -> Self {
        Self {}
    }
}

impl SegmentGenerator for RootSegment {
    fn output(&self, shell: Shell, theme: &Theme) -> Option<Segments> {
        let text = match shell {
            Shell::Bash => r" \$ ".into(),
            Shell::Zsh => todo!(),
            Shell::Bare => match unsafe { libc::getuid() } {
                0 => " # ".into(),
                _ => " $ ".into(),
            },
        };

        let (bg, fg) = match theme {
            Theme::Default => (BackgroundColor(130), ForegroundColor(255)),
        };

        Some(Segments::One(Segment {
            text,
            bg,
            fg,
            blinking: false,
        }))
    }
}
