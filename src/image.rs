use crate::point::Point;

/// A plain RGBA image.
///
/// An [`Image`] can be used either to load a regular image from the disk or to provide
/// a rendering surface before presenting the final result to the user, eg. Content generated
/// on another thread. [`Image`]s are **NOT** textures, and as a result, the GPU cannot be used
/// to accelerate rendering. This may impact performance for multiple or large images.
pub struct Image {
    position: Point,
    _pixels: Vec<u8>,
}

impl Image {
    #[cfg_attr(feature = "aggr_inline", inline)]
    pub fn new(bytes: Vec<u8>, position: Point) -> Self {
        Self {
            position,
            _pixels: bytes,
        }
    }

    /// Sets the position for the image. The position will **NOT** be the
    /// center of the image but the topleft corner of it.
    #[cfg_attr(feature = "aggr_inline", inline)]
    pub fn set_position(&mut self, position: Point) {
        self.position = position
    }

    #[cfg_attr(feature = "aggr_inline", inline)]
    pub fn draw(&self, _width: u64, _framebuffer: &mut [u8]) {
        todo!()
    }
}
