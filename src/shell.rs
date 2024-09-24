#[derive(Clone, Copy)]
pub enum Shell {
    Bash,
    Zsh,
    Bare,
}

impl TryFrom<&str> for Shell {
    type Error = Box<dyn std::error::Error>;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "auto" => todo!(),
            "bash" => Ok(Shell::Bash),
            "bare" => Ok(Shell::Bare),
            "zsh" => Ok(Shell::Zsh),
            _ => Err("unknown shell".into()),
        }
    }
}
