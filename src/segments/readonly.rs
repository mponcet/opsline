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
        let readonly = unsafe { libc::access(c".".as_ptr(), libc::W_OK) } != 0;

        if readonly {
            Some(Vec::from([Segment {
                name: "readonly",
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
