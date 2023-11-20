/// Plain old pixel point in 2D space. Pixel point means the point is at a fixed
/// location within your framebuffer (eg. x = 42, y = 51) so you will have to implement
/// scaling and resizing yourself.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq)]
pub struct Point {
    pub x: i64,
    pub y: i64,
}

/// Lightweight alternative to [`Point::draw`] (Does not depend on [`Point`] or any other datatype).
/// You can use this if you already have your points ready to be drawn and don't want to create new structs
/// for them.
/// # Parameters
/// - `color` -> Defines the color of the pixel. Only 8-bit RGBA values are allowed (Transparency is assumed but not required).
/// - `x` -> X (Horizontal) position of the pixel
/// - `y` -> y (Vertical) position of the pixel
/// - `width` -> The width of the framebuffer that the pixel will be drawn upon.
/// - `fb` -> A mutable reference to the framebuffer to write to.
#[cfg_attr(feature = "aggr_inline", inline)]
pub fn draw_pixel(color: [u8; 4], x: u64, y: u64, width: u64, fb: &mut [u8]) {
    let idx = ((y * width + x) as usize) * 4;
    fb[idx..idx + 4].copy_from_slice(&color)
}
