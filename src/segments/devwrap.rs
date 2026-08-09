use crate::segments::{Segment, SegmentSection};
use crate::shell::Shell;
use crate::theme::Theme;

pub struct DevwrapSegment;

impl DevwrapSegment {
    pub fn new() -> Self {
        Self {}
    }
}

impl Segment for DevwrapSegment {
    fn name(&self) -> &'static str {
        "devwrap"
    }

    fn output(&self, _shell: Shell, theme: &Theme) -> Option<Vec<super::SegmentSection>> {
        if std::env::var("DEVWRAP").is_ok() {
            Some(Vec::from([SegmentSection::Section {
                text: "󰕥".into(),
                bg: theme.devwrap_bg,
                fg: theme.devwrap_fg,
                blinking: false,
            }]))
        } else {
            None
        }
    }
}
