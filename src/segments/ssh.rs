use crate::fonts;
use crate::segments::{Segment, SegmentGenerator};
use crate::shell::Shell;
use crate::theme::{BackgroundColor, ForegroundColor, Theme};

pub struct SshSegment;

impl SshSegment {
    pub fn new() -> Self {
        Self {}
    }
}

impl SegmentGenerator for SshSegment {
    fn output(&self, _shell: Shell, theme: Theme) -> Option<Vec<super::Segment>> {
        let (bg, fg) = match theme {
            Theme::Default => (BackgroundColor(166), ForegroundColor(254)),
            Theme::Gruvbox => (BackgroundColor(96), ForegroundColor(229)),
        };

        if std::env::var("SSH_CLIENT").is_ok() {
            Some(Vec::from([Segment {
                text: format!(" {} ", fonts::NerdFonts::MD_SSH),
                bg,
                fg,
                blinking: false,
            }]))
        } else {
            None
        }
    }
}
