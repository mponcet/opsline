use std::fmt;

use crate::shell::Shell;

#[derive(Clone, Copy)]
pub struct BackgroundColor(pub u8);

impl fmt::Display for BackgroundColor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl BackgroundColor {
    pub fn fmt(&self, shell: Shell) -> impl fmt::Display {
        struct Helper(BackgroundColor, Shell);
        impl fmt::Display for Helper {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                match self.1 {
                    Shell::Bash => write!(f, r"\[\e[48;5;{}m\]", self.0),
                    Shell::Zsh => write!(f, "%{{\x1b[48;5;{}m%}}", self.0),
                }
            }
        }

        Helper(*self, shell)
    }
}

#[derive(Clone, Copy)]
pub struct ForegroundColor(pub u8);

impl From<BackgroundColor> for ForegroundColor {
    fn from(bg: BackgroundColor) -> Self {
        ForegroundColor(bg.0)
    }
}

impl fmt::Display for ForegroundColor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl ForegroundColor {
    pub fn fmt(&self, shell: Shell) -> impl fmt::Display {
        struct Helper(ForegroundColor, Shell);
        impl fmt::Display for Helper {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                match self.1 {
                    Shell::Bash => write!(f, r"\[\e[38;5;{}m\]", self.0),
                    Shell::Zsh => write!(f, "%{{\x1b[38;5;{}m%}}", self.0),
                }
            }
        }

        Helper(*self, shell)
    }
}

#[derive(Clone, Copy)]
pub struct Blink;

impl Blink {
    pub fn fmt(&self, shell: Shell) -> impl fmt::Display {
        struct Helper(Blink, Shell);
        impl fmt::Display for Helper {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                match self.1 {
                    Shell::Bash => write!(f, r"\[\e[5m\]"),
                    Shell::Zsh => write!(f, "%{{\x1b[5m%}}"),
                }
            }
        }

        Helper(*self, shell)
    }
}

#[derive(Clone, Copy)]
pub struct Reset;

impl Reset {
    pub fn fmt(&self, shell: Shell) -> impl fmt::Display {
        struct Helper(Reset, Shell);
        impl fmt::Display for Helper {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                match self.1 {
                    Shell::Bash => write!(f, r"\[\e[0m\]"),
                    Shell::Zsh => write!(f, "%{{\x1b[0m%}}"),
                }
            }
        }

        Helper(*self, shell)
    }
}

pub struct Theme {
    pub container_bg: BackgroundColor,
    pub container_fg: ForegroundColor,
    pub cwd_bg: BackgroundColor,
    pub cwd_fg: ForegroundColor,
    pub git_branch_bg: BackgroundColor,
    pub git_branch_fg: ForegroundColor,
    pub git_ahead_bg: BackgroundColor,
    pub git_ahead_fg: ForegroundColor,
    pub git_behind_bg: BackgroundColor,
    pub git_behind_fg: ForegroundColor,
    pub git_modified_bg: BackgroundColor,
    pub git_modified_fg: ForegroundColor,
    pub git_staged_bg: BackgroundColor,
    pub git_staged_fg: ForegroundColor,
    pub git_untracked_bg: BackgroundColor,
    pub git_untracked_fg: ForegroundColor,
    pub git_conflicted_bg: BackgroundColor,
    pub git_conflicted_fg: ForegroundColor,
    pub kube_context_bg: BackgroundColor,
    pub kube_context_fg: ForegroundColor,
    pub kube_namespace_bg: BackgroundColor,
    pub kube_namespace_fg: ForegroundColor,
    pub readonly_bg: BackgroundColor,
    pub readonly_fg: ForegroundColor,
    pub root_bg: BackgroundColor,
    pub root_fg: ForegroundColor,
    pub ssh_bg: BackgroundColor,
    pub ssh_fg: ForegroundColor,
}

mod default;
mod gruvbox;

impl TryFrom<&str> for Theme {
    type Error = Box<dyn std::error::Error>;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "default" => Ok(default::DEFAULT),
            "gruvbox" => Ok(gruvbox::GRUVBOX),
            _ => Err("unknown theme".into()),
        }
    }
}
