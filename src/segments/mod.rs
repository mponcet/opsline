use crate::{
    theme::Theme,
    theme::{BackgroundColor, ForegroundColor},
    Shell,
};

pub mod cwd;
pub mod git;
pub mod kube;
pub mod root;
pub mod ssh;

pub struct Segment {
    pub text: String,
    pub bg: BackgroundColor,
    pub fg: ForegroundColor,
    pub blinking: bool,
}

pub trait SegmentGenerator {
    fn output(&self, shell: Shell, theme: Theme) -> Option<Vec<Segment>>;
}
