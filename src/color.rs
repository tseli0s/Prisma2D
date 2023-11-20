/// Just an RGBA color. Works with about 99.9% of hardware.
/// The `a` field (Short for "Alpha", aka. Transparency) is only
/// available with the `transparency` feature.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    #[cfg(feature = "transparency")]
    pub a: u8,
}

impl Color {
    pub const WHITE: Color = Color {
        r: 0xff,
        g: 0xff,
        b: 0xff,
        #[cfg(feature = "transparency")]
        a: 0xff,
    };

    pub const BLACK: Color = Color {
        r: 0x0,
        g: 0x0,
        b: 0x0,
        #[cfg(feature = "transparency")]
        a: 0xff,
    };

    pub const RED: Color = Color {
        r: 0xff,
        g: 0x0,
        b: 0x0,
        #[cfg(feature = "transparency")]
        a: 0xff,
    };

    pub const GREEN: Color = Color {
        r: 0x0,
        g: 0xff,
        b: 0x0,
        #[cfg(feature = "transparency")]
        a: 0xff,
    };

    pub const BLUE: Color = Color {
        r: 0x0,
        g: 0x0,
        b: 0xff,
        #[cfg(feature = "transparency")]
        a: 0xff,
    };

    #[cfg_attr(feature = "aggr_inline", inline)]
    pub fn as_slice(&self) -> [u8; 4] {
        [
            self.r,
            self.g,
            self.b,
            #[cfg(feature = "transparency")]
            self.a,
            #[cfg(not(feature = "transparency"))]
            0xff,
        ]
    }
}

impl Default for Color {
    fn default() -> Self {
        Color::WHITE
    }
}

/// Clears the buffer given with the specified [`Color`].
///
/// This function is a fast way to completely clear the contents of the
/// framebuffer given. It uses chunking and multithreading (if enabled) to
/// speed up the function as much as possible.
///
/// # Examples
/// ```
/// // This is your framebuffer.
/// let mut framebuffer = vec![0; (1920 * 1080) * 4]; // Assuming RGBA format
/// prisma2d::color::clear_buffer(&mut framebuffer, Color::WHITE);
/// ```
pub fn clear_buffer(buffer: &mut [u8], color: Color) {
    #[cfg(feature = "threading")]
    clear_buffer_multithreaded(buffer, color);
    #[cfg(not(feature = "threading"))]
    clear_buffer_single_thread(buffer, color);
}

#[cfg(feature = "threading")]
fn clear_buffer_multithreaded(buffer: &mut [u8], color: Color) {
    use rayon::prelude::*;
    buffer.par_chunks_mut(4 * 8).for_each(|chunk| {
        let color_chunk = [color.r, color.g, color.b, color.a];
        for pixel in chunk.chunks_mut(4) {
            pixel.copy_from_slice(&color_chunk);
        }
    });
}

#[cfg(not(feature = "threading"))]
fn clear_buffer_single_thread(buffer: &mut [u8], color: Color) {
    for pixel in buffer.chunks_exact_mut(4) {
        pixel.copy_from_slice(&color.as_slice())
    }
}
