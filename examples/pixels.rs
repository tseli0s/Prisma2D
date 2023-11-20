#![deny(clippy::all)]
#![forbid(unsafe_code)]

use pixels::{Pixels, SurfaceTexture};
use prisma2d::color::Color;
use rand::Rng;
use std::process::abort;
use winit::dpi::LogicalSize;
use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;

const WIDTH: u32 = 1280;
const HEIGHT: u32 = 720;
const PIXELS_PER_SECOND: i32 = 1024576 * 2;

fn main() {
    env_logger::init();
    let mut rng = rand::thread_rng();
    let event_loop = EventLoop::new();
    let window = {
        let size = LogicalSize::new(WIDTH as f64, HEIGHT as f64);
        WindowBuilder::new()
            .with_title("Pixels - Prisma2D")
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

    event_loop.run(move |event, _, control_flow| {
        if let Event::RedrawRequested(_) = event {
            let fb = pixels.frame_mut();
            for _ in 0..PIXELS_PER_SECOND {
                let x = rng.gen_range(0..WIDTH);
                let y = rng.gen_range(0..HEIGHT);
                let r = rng.gen_range(0..255);
                let g = rng.gen_range(0..255);
                let b = rng.gen_range(0..255);
                let color = Color { r, g, b, a: 0xff };

                prisma2d::point::draw_pixel(color.as_slice(), x as u64, y as u64, WIDTH as u64, fb);
            }

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
