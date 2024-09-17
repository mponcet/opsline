use crate::segments::{Segment, SegmentGenerator, Segments};
use crate::theme::{BackgroundColor, ForegroundColor};
use crate::Shell;
use crate::Theme;

pub struct CwdSegment {
    dironly: bool,
}

impl Default for CwdSegment {
    fn default() -> Self {
        Self { dironly: true }
    }
}

impl CwdSegment {
    pub fn new(dironly: bool) -> Self {
        Self { dironly }
    }
}

impl SegmentGenerator for CwdSegment {
    fn output(&self, shell: Shell, theme: &Theme) -> Option<Segments> {
        let cwd = std::env::current_dir().unwrap_or_default();

        let text = if self.dironly {
            match shell {
                Shell::Bash => r" \W ".into(),
                Shell::Zsh => todo!(),
                Shell::Bare => cwd
                    .file_name()
                    .unwrap_or_default()
                    .to_string_lossy()
                    .into_owned(),
            }
        } else {
            match shell {
                Shell::Bash => r" \w ".into(),
                Shell::Zsh => todo!(),
                Shell::Bare => {
                    let Ok(Some(home)) = homedir::my_home() else {
                        panic!("Can't get home directory");
                    };

                    if cwd.starts_with(&home) {
                        format!(" ~/{} ", cwd.strip_prefix(home).unwrap().to_string_lossy())
                    } else {
                        cwd.to_string_lossy().into_owned()
                    }
                }
            }
        };

        let (bg, fg) = match theme {
            Theme::Default => (BackgroundColor(241), ForegroundColor(250)),
        };

        Some(Segments::One(Segment {
            text,
            bg,
            fg,
            blinking: false,
        }))
    }
}
