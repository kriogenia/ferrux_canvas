//! Ferrux Canvas is an abstraction layer over the [Pixels](https://crates.io/crates/pixels) crate.
//! It manages the pixel buffer exposing simple operations to draw pixels, lines and figures of one
//! color in the screen. In its current state it only works with [Winit](https://crates.io/crates/winit).
//!
//! # Building a canvas
//! Right now, the only [`Canvas`] provided is [`WinitCanvas`], which requires a [`Window`], which
//! will need itself an [`EventLoop`] reference.
//!
//! ```no_run
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let event_loop = winit::event_loop::EventLoop::new();
//! let window = winit::window::Window::new(&event_loop)?;
//! let canvas = ferrux_canvas::canvas::winit::WinitCanvas::new(&window)?;
//! # Ok(())}
//! ```
//!
//! # Running a canvas
//! The main flow to use a canvas is:
//!
//! * Use the drawing functions like [`draw_line`] and [`draw_triangle`].
//! * Call the [`render`] method to print it on screen.
//! * Use [`reset_frame`] to clear the current buffer and draw a new frame.
//!
//! The following example takes the [`WinitCanvas`] we built and draws a morphing triangle.
//! ```no_run
//! use ferrux_canvas::canvas::Canvas;
//! use ferrux_canvas::color;
//! use winit::event::Event;
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! # use ferrux_canvas::color::palette;
//! let event_loop = winit::event_loop::EventLoop::new();
//! # let window = winit::window::Window::new(&event_loop)?;
//! # let mut canvas = ferrux_canvas::canvas::winit::WinitCanvas::new(&window)?;
//! let mut x: i32 = 1;
//! let mut incrementing = true;
//!
//! event_loop.run(move |event, _, control_flow| {
//!   match event {
//!     Event::MainEventsCleared => {
//!       window.request_redraw();
//!     }
//!     Event::RedrawRequested(_) => {
//!       if !(1..100).contains(&x) {
//!         incrementing = !incrementing;
//!       }
//!       x += if incrementing { 1 } else { -1 };
//!       canvas.draw_triangle((100, (100 - x) as u32), (100 - x as u32, 100),
//!                            (200 - x as u32, 200 - x as u32), palette::WHITE);
//!       canvas.render().unwrap();
//!       canvas.reset_frame();
//!     }
//!     _ => (),
//!   }
//! });
//! # Ok(()) }
//! ```
//!
//! [`Canvas`]: canvas::Canvas
//! [`draw_line`]: canvas::Canvas::draw_line
//! [`draw_triangle`]: canvas::Canvas::draw_triangle
//! [`render`]: canvas::Canvas::render
//! [`reset_frame`]: canvas::Canvas::reset_frame
//! [`WinitCanvas`]: canvas::winit::WinitCanvas
//! [`Window`]: winit::window::Window
//! [`EventLoop`]: winit::event_loop::EventLoop
//!

pub mod canvas;
pub mod color;

extern crate winit;