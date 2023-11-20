//! # Prisma2D
//! **Prisma2D** is a crate to wrap all the mathematics behind drawing basic primitives
//! such as points, lines and circles in Rust. All it needs is a framebuffer to draw upon
//! (Represented in the crate with a `&mut [u8]`) to output the results. As such, it is
//! completely API/library independent.
//!
//! One of the main limitations of this crate is the focus on performance over quality. If you
//! plan on using it on 2D games, you should first ensure you like the output graphics, otherwise
//! [`wgpu`](https://github.com/gfx-rs/wgpu) will be of more use.
//!
//! # Features
//! - `nostd` - Disables the use of the Rust standard library.
//! - `threading` - Enables multithreading using the `rayon` library.
//! - `transparency` - Adds transparency support for [`crate::color::Color`]
//! - `images` - Enables support for image buffers (You will still need an external crate to load the image)
//! - `aggr-inline` - Inlines multiple functions within the crate such as constructors and setters.

#![cfg_attr(target_feature = "nostd", no_std)]
#![deny(clippy::all)]
#![forbid(unsafe_code)]

/// Circles
pub mod circle;
/// Colors
pub mod color;
/// Images
#[cfg(feature = "images")]
pub mod image;
/// Plain line
pub mod line;
/// Pixel points
pub mod point;
/// Vertices, can be used for more complex shapes.
pub mod vertices;
