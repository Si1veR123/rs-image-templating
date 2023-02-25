
#[derive(Debug, Clone)]
pub struct RGBAColor(pub f64, pub f64, pub f64, pub f64);

pub const RED: RGBAColor = RGBAColor(255.0, 0.0, 0.0, 0.0);
pub const GREEN: RGBAColor = RGBAColor(0.0, 255.0, 0.0, 0.0);
pub const BLUE: RGBAColor = RGBAColor(0.0, 0.0, 255.0, 0.0);
pub const WHITE: RGBAColor = RGBAColor(255.0, 255.0, 255.0, 0.0);
pub const BLACK: RGBAColor = RGBAColor(0.0, 0.0, 0.0, 0.0);
pub const YELLOW: RGBAColor = RGBAColor(255.0, 255.0, 0.0, 0.0);
pub const CYAN: RGBAColor = RGBAColor(0.0, 255.0, 255.0, 0.0);
pub const MAGENTA: RGBAColor = RGBAColor(255.0, 0.0, 255.0, 0.0);
pub const SILVER: RGBAColor = RGBAColor(192.0, 192.0, 192.0, 0.0);
pub const GREY: RGBAColor = RGBAColor(128.0, 128.0, 128.0, 0.0);
pub const DARK_RED: RGBAColor = RGBAColor(128.0, 0.0, 0.0, 0.0);
pub const DARK_GREEN: RGBAColor = RGBAColor(0.0, 128.0, 0.0, 0.0);
pub const DARK_BLUE: RGBAColor = RGBAColor(0.0, 0.0, 128.0, 0.0);
pub const ORANGE: RGBAColor = RGBAColor(255.0, 165.0, 0.0, 0.0);
pub const PURPLE: RGBAColor = RGBAColor(128.0, 0.0, 128.0, 0.0);
pub const LIGHT_GREY: RGBAColor = RGBAColor(211.0, 211.0, 211.0, 0.0);
