use crate::Shell;
use crate::configuration::CwdConfiguration;
use crate::segments::{Segment, SegmentGenerator};
use crate::theme::Theme;

pub struct CwdSegment<'a> {
    config: &'a CwdConfiguration,
}

impl<'a> CwdSegment<'a> {
    pub fn new(config: &'a CwdConfiguration) -> Self {
        Self { config }
    }
}

impl SegmentGenerator for CwdSegment<'_> {
    fn name(&self) -> &'static str {
        "cwd"
    }

    fn output(&self, shell: Shell, theme: &Theme) -> Option<Vec<Segment>> {
        let text = if self.config.dironly {
            match shell {
                Shell::Bash => r" \W ".into(),
                Shell::Zsh => " %1d ".into(),
                Shell::Fish => {
                    let cwd = std::env::current_dir().ok()?;
                    let cwd = cwd.file_name()?.to_string_lossy();
                    format!(" {cwd} ").into()
                }
            }
        } else {
            match shell {
                Shell::Bash => r" \w ".into(),
                Shell::Zsh => " %d ".into(),
                Shell::Fish => {
                    let home = std::env::home_dir()?;
                    let cwd = std::env::current_dir().ok()?;
                    if cwd == home {
                        " ~ ".into()
                    } else if let Ok(cwd) = cwd.strip_prefix(home) {
                        format!(" ~/{} ", cwd.to_string_lossy()).into()
                    } else {
                        format!(" {} ", cwd.to_string_lossy()).into()
                    }
                }
            }
        };

        Some(Vec::from([Segment {
            name: "cwd",
            text,
            bg: theme.cwd_bg,
            fg: theme.cwd_fg,
            blinking: false,
        }]))
    }
}
