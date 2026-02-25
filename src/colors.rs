/// The library provides a variety of basic colors for quick use but imposes no limits on the
/// choice of colors used. Any RGBA color can be used as long as it follows this structure.
use image::Rgba;

pub type Color = Rgba<u8>;

pub const YELLOW: Color = Rgba([255, 255, 0, 255]);
pub const GREEN: Color = Rgba([0, 176, 80, 255]);
pub const GRAY: Color = Rgba([191, 191, 191, 255]);
pub const RED: Color = Rgba([255, 0, 0, 255]);
pub const BLUE: Color = Rgba([0, 112, 192, 255]);
pub const ORANGE: Color = Rgba([255, 153, 0, 255]);
pub const PURPLE: Color = Rgba([112, 48, 160, 255]);
pub const WHITE: Color = Rgba([255, 255, 255, 255]);
pub const BLACK: Color = Rgba([0, 0, 0, 255]);
