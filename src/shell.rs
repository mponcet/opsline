#[derive(Clone, Copy)]
pub enum Shell {
    Bash,
    Zsh,
}

impl TryFrom<&str> for Shell {
    type Error = Box<dyn std::error::Error>;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "bash" => Ok(Shell::Bash),
            "zsh" => Ok(Shell::Zsh),
            _ => Err("unknown shell".into()),
        }
    }
}
