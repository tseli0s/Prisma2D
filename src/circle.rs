use crate::{color::Color, point::Point};

pub struct Circle {
    radius: i64,
    position: Point,
    color: Color,
}

impl Circle {
    #[cfg_attr(feature = "aggr_inline", inline)]
    pub fn new(center: Point, size: i64, color: Option<Color>) -> Self {
        Self {
            position: center,
            radius: size,
            color: color.unwrap_or(Color::default()),
        }
    }

    #[cfg_attr(inline_aggr, inline)]
    pub fn set_size(&mut self, size: i64) {
        self.radius = size
    }

    pub fn draw(&self, framebuffer: &mut [u8], width: u64) {
        let mut x = 0;
        let mut y = self.radius;
        let mut d = 3 - 2 * self.radius;

        while x <= y {
            Self::draw_circle_points(self.color, self.position, x, y, framebuffer, width);
            x += 1;

            if d < 0 {
                d += 4 * x + 6;
            } else {
                d += 4 * (x - y) + 10;
                y -= 1;
            }
        }
    }

    fn draw_circle_points(
        color: Color,
        center: Point,
        x: i64,
        y: i64,
        framebuffer: &mut [u8],
        width: u64,
    ) {
        crate::point::draw_pixel(
            color.as_slice(),
            (center.x + x) as u64,
            (center.y + y) as u64,
            width,
            framebuffer,
        );
        crate::point::draw_pixel(
            color.as_slice(),
            (center.x - x) as u64,
            (center.y + y) as u64,
            width,
            framebuffer,
        );
        crate::point::draw_pixel(
            color.as_slice(),
            (center.x + x) as u64,
            (center.y - y) as u64,
            width,
            framebuffer,
        );
        crate::point::draw_pixel(
            color.as_slice(),
            (center.x - x) as u64,
            (center.y - y) as u64,
            width,
            framebuffer,
        );
        crate::point::draw_pixel(
            color.as_slice(),
            (center.x + y) as u64,
            (center.y + x) as u64,
            width,
            framebuffer,
        );
        crate::point::draw_pixel(
            color.as_slice(),
            (center.x - y) as u64,
            (center.y + x) as u64,
            width,
            framebuffer,
        );
        crate::point::draw_pixel(
            color.as_slice(),
            (center.x + y) as u64,
            (center.y - x) as u64,
            width,
            framebuffer,
        );
        crate::point::draw_pixel(
            color.as_slice(),
            (center.x - y) as u64,
            (center.y - x) as u64,
            width,
            framebuffer,
        );
    }
}
