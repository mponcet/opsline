use crate::theme::{BackgroundColor, ForegroundColor, Theme};
use crate::Shell;

pub mod containers;
pub mod cwd;
pub mod git;
pub mod kube;
pub mod readonly;
pub mod root;
pub mod ssh;

pub use containers::ContainersSegment;
pub use cwd::CwdSegment;
pub use git::GitSegment;
pub use kube::KubeSegment;
pub use readonly::ReadonlySegment;
pub use root::RootSegment;
pub use ssh::SshSegment;

pub struct Segment {
    pub text: String,
    pub bg: BackgroundColor,
    pub fg: ForegroundColor,
    pub blinking: bool,
}

pub trait SegmentGenerator {
    fn output(&self, shell: Shell, theme: Theme) -> Option<Vec<Segment>>;
}
