use std::os::raw::c_char;

use crate::fonts;
use crate::segments::{Segment, SegmentGenerator};
use crate::shell::Shell;
use crate::theme::Theme;
use crate::theme::{BackgroundColor, ForegroundColor};

pub struct ReadonlySegment;

impl ReadonlySegment {
    pub fn new() -> Self {
        Self {}
    }
}

impl SegmentGenerator for ReadonlySegment {
    fn output(&self, _shell: Shell, theme: Theme) -> Option<Vec<Segment>> {
        let readonly = unsafe { libc::access(".\0".as_ptr() as *const c_char, libc::W_OK) } != 0;

        let (bg, fg) = match theme {
            Theme::Default => (BackgroundColor(124), ForegroundColor(254)),
            Theme::Gruvbox => (BackgroundColor(167), ForegroundColor(229)),
        };

        if readonly {
            Some(Vec::from([Segment {
                text: format!(" {} ", fonts::NerdFonts::FA_LOCK),
                bg,
                fg,
                blinking: false,
            }]))
        } else {
            None
        }
    }
}
