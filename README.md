<div align="center">
<h1>Prisma2D: Ultra-fast CPU 2D graphics</h1>
<img src="https://img.shields.io/crates/l/prisma2d?logoColor=white&color=blue&
" alt="License">
<img src="https://img.shields.io/github/repo-size/tseli0s/Prisma2D?logo=github&logoColor=white&label=repo%20size&labelColor=black&color=green
" alt="Repo Size">
</div>

Prisma2D is a blazingly fast, efficient yet minimal crate for basic 2D graphics on the CPU. for Rust. With Prisma2D, whether you're building games, simulations, or any graphics-intensive application, you can squeeze every single bit of processing power available to draw efficiently anything you want on the screen!

# Features
- 🦀 **No dependencies**: Prisma2D can be used with absolutely no dependencies (Not even `std`!). This can come in handy when you work in a very restricted environment such as low-level graphics. It will also never depend on any graphics hardware to be present and runs all logic on the CPU.

- 🚀 **Efficiency at its Core**: Prisma2D is designed for speed, being able to draw millions of pixels every second without struggling at all. It's optimized to handle the most demanding graphics tasks without breaking a sweat. Your primitives can be rendered with near-instant results, thanks to the use of efficient algorithms and extensive optimization.

- 🖌️ **Pixel Perfection**: Achieve pixel perfection effortlessly. Prisma2D provides precise control over pixels, allowing you to create crisp and clear graphics with absolute control on every single pixel.

# Getting Started
- Add Prisma2D to your Cargo.toml:

```toml
[dependencies]
prisma2d = "0.1.0"
```

- Draw literally anything (See [examples](./examples/) for more details):
```rust
use prisma2d::{
    line::Line,
    point::Point,
    color::Color,
};

fn main() {
    /* Create your framebuffer here. You can use a crate like 'pixels' */
    let framebuffer = Vec::<u8>::new();
    let line = Line::new(
        Point { x: 50, y: 200 },
        Point { x: 150, y: 200 },
        Color::default(), // White color by default
    )
    line.draw();
}
```

# TODOs
- [x] Lines
- [x] Pixels
- [x] Vertices
- [x] Examples
- [ ] Images
- [x] Multithreading
- [x] Helper functions
- [ ] Rectangles
- [ ] Filling
- [ ] Gradients

# FAQ
- *Does this support x?*\
Yes, it probably does, given that most operating systems work with a (shared) framebuffer to display the contents of a window, which is what Prisma2D wants to render upon.

- *How does this compare to other crates?*\
Before you get an answer, you must understand that this crate is in early development and even very basic features are missing, so I won't consider this factor for now.

* `tiny-skia` is close enough, and given the amount of time it had to develop, it can fulfill most of the functionality Prisma2D offers. (TODO: Benchmarks)
* `cairo` is a C library, so it can easily escape Rust's enforcing safety, however it's stable and has been used for many other libraries.
* `skia` is a C++ library (Meaning you can't use it from another language AT ALL), and it's bloated as hell (It can take hours and GBs of space to compile it), not to mention the resulting library is ~5MBs in space.

In essence, this crate tries to sit somewhere in a small corner of your codebase, whereas most crates try to gain complete control over your graphics rendering.

- *If it's so small why not just reimplement it by hand myself?*\
You will waste a few days to rewrite this crate, which you could've invested in your actual application/library. Also, you will have to learn some concepts from scratch just to reinvent the wheel.

- *I want x, please add it.*\
The code is here, the pull requests are there, make one and add anything you wish to add. If you don't know how to implement x, [open an issue](https://github.com/tseli0s/Prisma2D/issues/).

# Contributing

We welcome contributions! Whether it's bug reports, feature requests, or code contributions, every bit helps. Check out our Contribution Guidelines to get started.

Prisma2D is licensed under the MIT License - see the LICENSE file for details.