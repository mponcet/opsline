use crate::segments::{Segment, SegmentSection};
use crate::shell::Shell;
use crate::theme::Theme;

pub struct AwsSegment;

impl AwsSegment {
    pub fn new() -> Self {
        Self {}
    }
}

impl Segment for AwsSegment {
    fn name(&self) -> &'static str {
        "aws"
    }

    fn output(&self, _shell: Shell, theme: &Theme) -> Option<Vec<SegmentSection>> {
        let aws_profile = std::env::var("AWS_PROFILE").ok()?;

        let sections = Vec::from([SegmentSection {
            name: "aws",
            text: format!("  {} ", aws_profile).into(),
            bg: theme.aws_bg,
            fg: theme.aws_fg,
            blinking: false,
        }]);

        Some(sections)
    }
}
