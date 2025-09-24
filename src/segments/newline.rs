use crate::Shell;
use crate::Theme;
use crate::{SegmentGenerator, segments::Segment};

pub struct NewLineSegment;

impl NewLineSegment {
    pub fn new() -> Self {
        Self {}
    }
}

impl SegmentGenerator for NewLineSegment {
    fn output(&self, _shell: Shell, _theme: &Theme) -> Option<Vec<Segment>> {
        Some(Vec::from([Segment {
            name: "newline",
            text: "\n".into(),
            // not needed
            bg: crate::theme::BackgroundColor(0),
            // not needed
            fg: crate::theme::ForegroundColor(0),
            blinking: false,
        }]))
    }
}
