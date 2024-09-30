use crate::configuration::CwdConfiguration;
use crate::segments::{Segment, SegmentGenerator};
use crate::theme::{BackgroundColor, ForegroundColor};
use crate::Shell;
use crate::Theme;

pub struct CwdSegment<'a> {
    config: Option<&'a CwdConfiguration>,
}

impl<'a> CwdSegment<'a> {
    pub fn new(config: Option<&'a CwdConfiguration>) -> Self {
        Self { config }
    }
}

impl<'a> SegmentGenerator for CwdSegment<'a> {
    fn output(&self, shell: Shell, theme: Theme) -> Option<Vec<Segment>> {
        let cwd = std::env::current_dir().unwrap_or_default();

        let text = if self.config.unwrap_or(&CwdConfiguration::default()).dironly {
            match shell {
                Shell::Bash => r" \W ".into(),
                Shell::Zsh => " %1d ".into(),
                Shell::Bare => cwd
                    .file_name()
                    .unwrap_or_default()
                    .to_string_lossy()
                    .into_owned(),
            }
        } else {
            match shell {
                Shell::Bash => r" \w ".into(),
                Shell::Zsh => " %d ".into(),
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
            Theme::Gruvbox => (BackgroundColor(66), ForegroundColor(250)),
        };

        Some(Vec::from([Segment {
            text,
            bg,
            fg,
            blinking: false,
        }]))
    }
}
