#![deny(clippy::all)]
#![forbid(unsafe_code)]

use pixels::{Pixels, SurfaceTexture};
use prisma2d::circle::Circle;
use prisma2d::color::Color;
use prisma2d::point::Point;
use std::process::abort;
use winit::dpi::LogicalSize;
use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;

const WIDTH: u32 = 640;
const HEIGHT: u32 = 480;
const CIRCLE_SIZE: i64 = 128;

fn main() {
    env_logger::init();
    let event_loop = EventLoop::new();
    let window = {
        let size = LogicalSize::new(WIDTH as f64, HEIGHT as f64);
        WindowBuilder::new()
            .with_title("Circle - Prisma2D")
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

    let mut size = CIRCLE_SIZE + (CIRCLE_SIZE / 2);
    let mut circle = Circle::new(
        Point {
            x: (WIDTH / 2) as i64,
            y: (HEIGHT / 2) as i64,
        },
        CIRCLE_SIZE,
        None,
    );
    event_loop.run(move |event, _, control_flow| {
        if let Event::RedrawRequested(_) = event {
            if size >= 1 {
                size -= 1
            } else {
                size = CIRCLE_SIZE
            }
            prisma2d::color::clear_buffer(pixels.frame_mut(), Color::BLACK);
            circle.set_size(size);
            circle.draw(pixels.frame_mut(), WIDTH as u64);

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
