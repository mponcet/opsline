use std::borrow::Cow;

use crate::Shell;
use crate::theme::{BackgroundColor, ForegroundColor, Theme};

pub mod aws;
pub mod containers;
pub mod cwd;
pub mod devwrap;
pub mod git;
pub mod kube;
pub mod readonly;
pub mod root;
pub mod ssh;
pub mod terraform;

pub use containers::ContainersSegment;
pub use cwd::CwdSegment;
pub use devwrap::DevwrapSegment;
pub use git::GitSegment;
pub use kube::KubeSegment;
pub use readonly::ReadonlySegment;
pub use root::RootSegment;
pub use ssh::SshSegment;
pub use terraform::TerraformSegment;

pub enum SegmentSection {
    Section {
        text: Cow<'static, str>,
        bg: BackgroundColor,
        fg: ForegroundColor,
        blinking: bool,
    },
    Seperator,
}

pub trait Segment {
    fn name(&self) -> &'static str;
    fn output(&self, shell: Shell, theme: &Theme) -> Option<Vec<SegmentSection>>;
}
