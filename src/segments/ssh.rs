use crate::segments::{Segment, SegmentSection};
use crate::shell::Shell;
use crate::theme::Theme;

pub struct SshSegment;

impl SshSegment {
    pub fn new() -> Self {
        Self {}
    }
}

impl Segment for SshSegment {
    fn name(&self) -> &'static str {
        "ssh"
    }

    fn output(&self, _shell: Shell, theme: &Theme) -> Option<Vec<super::SegmentSection>> {
        if std::env::var("SSH_CLIENT").is_ok() {
            Some(Vec::from([SegmentSection::Section {
                text: "󰣀".into(),
                bg: theme.ssh_bg,
                fg: theme.ssh_fg,
                blinking: false,
            }]))
        } else {
            None
        }
    }
}
