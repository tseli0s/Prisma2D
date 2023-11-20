#![deny(clippy::all)]
#![forbid(unsafe_code)]

use pixels::{Pixels, SurfaceTexture};
use prisma2d::color::Color;
use prisma2d::vertices::Vertex;
use std::process::abort;
use winit::dpi::LogicalSize;
use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;

const WIDTH: u32 = 640;
const HEIGHT: u32 = 480;

fn main() {
    env_logger::init();
    let event_loop = EventLoop::new();
    let window = {
        let size = LogicalSize::new(WIDTH as f64, HEIGHT as f64);
        WindowBuilder::new()
            .with_title("Triangle - Prisma2D")
            .with_inner_size(size)
            .with_min_inner_size(size)
            .build(&event_loop)
            .unwrap()
    };

    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(WIDTH, HEIGHT, surface_texture).unwrap_or_else(|e| {
            log::error!("Creating framebuffer failed: {}", e.to_string());
            abort()
        })
    };

    let w = WIDTH  as i64;
    let h = HEIGHT as i64;
    /*
     * One line takes two vertices, so for a triangle which needs 3 lines (because of three sides)
     * that should be 3 * 2 = 6 vertices in total.
     * We put them in an array to use with prisma2d::vertices::draw_vertices()
     */
    let vertices = [
        Vertex::new(w / 2, 0),
        Vertex::new(0, h - 1),

        Vertex::new(0, h - 1),
        Vertex::new(w - 1, h - 1),

        Vertex::new(w - 1, h - 1),
        Vertex::new(w / 2, 0),
    ];
    event_loop.run(move |event, _, control_flow| {
        if let Event::RedrawRequested(_) = event {
            let framebuffer = pixels.frame_mut();
            prisma2d::vertices::draw_vertices(&vertices, Color::WHITE, w, framebuffer);

            if let Err(_) = pixels.render() {
                *control_flow = ControlFlow::Exit;
                return;
            };
        }

        if let Event::WindowEvent { event, .. } = event {
            if let WindowEvent::Resized(size) = event {
                pixels
                    .resize_surface(size.width, size.height)
                    .unwrap_or_else(|e| {
                        log::error!("Failed to resize pixels' surface: {}", e.to_string());
                        abort()
                    })
            }

            if let WindowEvent::CloseRequested = event {
                *control_flow = ControlFlow::Exit;
            }
        }

        window.request_redraw();
    });
}
