use crate::{color::Color, point::Point, line::Line};

/// A very basic vertex.
/// 
/// A vertex is simply a single point in the framebuffer, optionally carrying [`Color`] information.
/// A vertex can be used to draw more complicated shapes, eg. Polygons.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Vertex {
    pub point: Point,
}

impl Vertex {
    #[cfg_attr(feature = "aggr_inline", inline)]
    pub fn new(x: i64, y: i64) -> Self {
        Self {
            point: Point { x, y }
        }
    }
}

impl From<Point> for Vertex {
    fn from(point: Point) -> Self {
        Self { point }
    }
}

impl Into<Point> for Vertex {
    fn into(self) -> Point {
        self.point
    }
}

pub fn draw_vertices(vertices: &[Vertex], color: Color, width: i64, framebuffer: &mut [u8]) {
    assert!(vertices.len() >= 2, "You must have at least 2 vertices to draw! (Use prisma2d::point::Point for a single pixel)");

    for vertices in vertices.chunks_exact(2) {
        let line = Line::new(vertices[0].point, vertices[1].point, Some(color));
        line.draw(width, framebuffer)
    }
}