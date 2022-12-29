use std::fmt::Display;

pub const CONTROL_SEQUENCE_INTRODUCER: &str = "\x1b[";
pub const RESET: &str = "0m";
pub const _CLEAR_SCREEN: &str = "2J";

pub struct TextColorRGB(pub u8, pub u8, pub u8);

impl Display for TextColorRGB {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "38;2;{};{};{}m", self.0, self.1, self.2)
    }
}

pub struct BackgroundColorRGB(pub u8, pub u8, pub u8);

impl Display for BackgroundColorRGB {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "48;2;{};{};{}m", self.0, self.1, self.2)
    }
}
