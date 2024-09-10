use crate::{
    theme::Theme,
    theme::{BackgroundColor, ForegroundColor},
    Shell,
};

pub mod cwd;
pub mod kube;
pub mod root;

pub struct SegmentOutput {
    pub text: String,
    pub bg: BackgroundColor,
    pub fg: ForegroundColor,
}

pub trait Segment {
    fn output(&self, shell: Shell, theme: &Theme) -> Option<SegmentOutput>;
}
