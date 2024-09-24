#[derive(Clone, Copy)]
pub struct BackgroundColor(pub u8);

#[allow(unused)]
impl BackgroundColor {
    pub const BLACK: Self = Self(40);
    pub const RED: Self = Self(41);
    pub const GREEN: Self = Self(42);
    pub const YELLOW: Self = Self(43);
    pub const BLUE: Self = Self(44);
    pub const PURPLE: Self = Self(45);
    pub const CYAN: Self = Self(46);
    pub const WHITE: Self = Self(47);
}

impl std::fmt::Display for BackgroundColor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Clone, Copy)]
pub struct ForegroundColor(pub u8);

#[allow(unused)]
impl ForegroundColor {
    pub const BLACK: Self = Self(30);
    pub const RED: Self = Self(31);
    pub const GREEN: Self = Self(32);
    pub const YELLOW: Self = Self(33);
    pub const BLUE: Self = Self(34);
    pub const PURPLE: Self = Self(35);
    pub const CYAN: Self = Self(36);
    pub const WHITE: Self = Self(37);
}

impl std::fmt::Display for ForegroundColor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Clone, Copy)]
pub enum Theme {
    Default,
}

impl TryFrom<&str> for Theme {
    type Error = Box<dyn std::error::Error>;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "default" => Ok(Theme::Default),
            _ => Err("unknown theme".into()),
        }
    }
}
