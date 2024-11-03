use std::os::raw::c_char;

use crate::segments::{Segment, SegmentGenerator};
use crate::shell::Shell;
use crate::theme::Theme;

pub struct ReadonlySegment;

impl ReadonlySegment {
    pub fn new() -> Self {
        Self {}
    }
}

impl SegmentGenerator for ReadonlySegment {
    fn output(&self, _shell: Shell, theme: &Theme) -> Option<Vec<Segment>> {
        let readonly = unsafe { libc::access(".\0".as_ptr() as *const c_char, libc::W_OK) } != 0;

        if readonly {
            Some(Vec::from([Segment {
                text: "  ".into(),
                bg: theme.readonly_bg,
                fg: theme.readonly_fg,
                blinking: false,
            }]))
        } else {
            None
        }
    }
}
