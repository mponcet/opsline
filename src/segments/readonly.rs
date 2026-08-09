use crate::segments::{Segment, SegmentSection};
use crate::shell::Shell;
use crate::theme::Theme;

pub struct ReadonlySegment;

impl ReadonlySegment {
    pub fn new() -> Self {
        Self {}
    }
}

impl Segment for ReadonlySegment {
    fn name(&self) -> &'static str {
        "readonly"
    }

    fn output(&self, _shell: Shell, theme: &Theme) -> Option<Vec<SegmentSection>> {
        let readonly = unsafe { libc::access(c".".as_ptr(), libc::W_OK) } != 0;

        if readonly {
            Some(Vec::from([SegmentSection::Section {
                text: "".into(),
                bg: theme.readonly_bg,
                fg: theme.readonly_fg,
                blinking: false,
            }]))
        } else {
            None
        }
    }
}
