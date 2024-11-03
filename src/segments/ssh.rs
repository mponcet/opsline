use crate::segments::{Segment, SegmentGenerator};
use crate::shell::Shell;
use crate::theme::Theme;

pub struct SshSegment;

impl SshSegment {
    pub fn new() -> Self {
        Self {}
    }
}

impl SegmentGenerator for SshSegment {
    fn output(&self, _shell: Shell, theme: &Theme) -> Option<Vec<super::Segment>> {
        if std::env::var("SSH_CLIENT").is_ok() {
            Some(Vec::from([Segment {
                text: " 󰣀 ".into(),
                bg: theme.ssh_bg,
                fg: theme.ssh_fg,
                blinking: false,
            }]))
        } else {
            None
        }
    }
}
