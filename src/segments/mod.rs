use std::borrow::Cow;

use crate::Shell;
use crate::theme::{BackgroundColor, ForegroundColor, Theme};

pub mod containers;
pub mod cwd;
pub mod git;
pub mod kube;
pub mod newline;
pub mod readonly;
pub mod root;
pub mod ssh;
pub mod terraform;

pub use containers::ContainersSegment;
pub use cwd::CwdSegment;
pub use git::GitSegment;
pub use kube::KubeSegment;
pub use newline::NewLineSegment;
pub use readonly::ReadonlySegment;
pub use root::RootSegment;
pub use ssh::SshSegment;
pub use terraform::TerraformSegment;

pub struct Segment {
    pub text: Cow<'static, str>,
    pub bg: BackgroundColor,
    pub fg: ForegroundColor,
    pub blinking: bool,
}

pub trait SegmentGenerator {
    fn output(&self, shell: Shell, theme: &Theme) -> Option<Vec<Segment>>;
}
