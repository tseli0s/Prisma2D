use crate::{color::Color, point::Point};
//#[cfg(feature = "threading")]
//use rayon::prelude::*;
#[derive(Debug, Clone, PartialEq, PartialOrd, Eq)]
pub struct Line {
    pub start: Point,
    pub end: Point,
    pub color: Color,
}

impl Line {
    #[cfg_attr(feature = "aggr_inline", inline)]
    pub fn new(start: Point, end: Point, color: Option<Color>) -> Self {
        return Self {
            start,
            end,
            color: color.unwrap_or(Color::WHITE),
        };
    }

    #[cfg_attr(feature = "aggr_inline", inline)]
    pub fn set_color(&mut self, c: Color) {
        self.color = c
    }

    fn bresenham(&self, width: i64, framebuffer: &mut [u8]) {
        let Line {
            mut start,
            end,
            color,
        } = self;

        let dx = i64::abs(end.x - start.x);
        let dy = i64::abs(end.y - start.y);

        let sx = if start.x < end.x { 1 } else { -1 };
        let sy = if start.y < end.y { 1 } else { -1 };

        let mut err = dx - dy;

        while start != *end {
            let index = ((start.y * width + start.x) as usize) * 4;
            framebuffer[index..index + 4].copy_from_slice(&color.as_slice());

            let double_err = err * 2;

            if double_err > -dy {
                err -= dy;
                start.x += sx;
            }

            if double_err < dx {
                err += dx;
                start.y += sy;
            }
        }
    }

    /*
    #[cfg(feature = "threading")]
    fn bresenham_multithreaded(&self, width: i64, framebuffer: &mut [u8]) {
        let Line { mut start, end, color } = self;

        let dx = i64::abs(end.x - start.x);
        let dy = i64::abs(end.y - start.y);

        let sx = if start.x < end.x { 1 } else { -1 };
        let sy = if start.y < end.y { 1 } else { -1 };

        let err = dx - dy;

        (0..((dx + dy) as usize)).into_par_iter().for_each(move |_| {
            let index = (start.y * width + start.x) as usize;
            framebuffer[index..index + 4].copy_from_slice(&color.as_slice());

            let double_err = err * 2;

            if double_err > -dy {
                err -= dy;
                start.x += sx;
            }

            if double_err < dx {
                err += dx;
                start.y += sy;
            }
        });
    }*/

    /// Draws the line (`self`) on the framebuffer. It does so by utilizing **Bresenham's algorithm**
    /// which is a very efficient algorithm to draw a single line on the monitor's display. This algorithm
    /// is widespread in low-level systems programming for its speed, however output quality is not as good as other algorithms.
    ///
    /// In the future, you will be able to choose what algorithm to use to draw the line through a `cargo`
    /// feature. For now I've only implemented the Bresenham one because it's fairly easy :)
    #[cfg_attr(feature = "aggr_inline", inline)]
    pub fn draw(&self, width: i64, framebuffer: &mut [u8]) {
        #[cfg(not(feature = "threading"))]
        self.bresenham(width, framebuffer);
        #[cfg(feature = "threading")]
        //self.bresenham_multithreaded(width, framebuffer);
        self.bresenham(width, framebuffer);
    }

    /// Draws the line to the screen, but also takes ownership of `self`
    /// and drops it after the function is done. Just a shortcut to `line.draw()` and `drop(line)`.
    #[cfg_attr(feature = "aggr_inline", inline)]
    pub fn draw_once(self, width: i64, framebuffer: &mut [u8]) {
        self.draw(width, framebuffer)
    }
}
