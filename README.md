[![Rust](https://github.com/kriogenia/ferrux_canvas/actions/workflows/rust.yml/badge.svg?branch=main)](https://github.com/kriogenia/ferrux_canvas/actions/workflows/rust.yml)

# FerruX Canvas

Ferrux Canvas is an abstraction layer over the [Pixels](https://crates.io/crates/pixels) crate. It manages the pixel
buffer exposing simple operations to draw pixels, lines and figures of one color in the screen. In its current state 
it only works with [Winit](https://crates.io/crates/winit).

The list of current goals is listed in the repository's [project](https://github.com/kriogenia/ferrux_canvas/projects/1).

## Usage

### Building a canvas
Right now, the only **Canvas** provided is **WinitCanvas**, which requires a **winit Window**, which will need itself an
**EventLoop** reference.

```rust
let event_loop = winit::event_loop::EventLoop::new();
let window = winit::window::Window::new(&event_loop)?;
let canvas = ferrux_canvas::canvas::winit::WinitCanvas::new(&window)?;
```

### Running a canvas
The main flow to use a canvas is:
* Use the drawing functions like [`draw_line`] and [`draw_triangle`].
* Call the [`render`] method to print it on screen.
* Use [`reset_frame`] to clear the current buffer and draw a new frame.

The following example takes the [`WinitCanvas`] we built and draws a morphing triangle.
```rust
let mut x: i32 = 1;
let mut incrementing = true;

event_loop.run(move |event, _, control_flow| {
   match event {
     Event::MainEventsCleared => {
       window.request_redraw();
     }
     Event::RedrawRequested(_) => {
       if !(1..100).contains(&x) {
         incrementing = !incrementing;
       }
       x += if incrementing { 1 } else { -1 };
       canvas.draw_triangle((100, (100 - x) as u32), (100 - x as u32, 100), (200 - x as u32, 200 - x as u32));
       canvas.render().unwrap();
       canvas.reset_frame();
     }
     _ => (),
   }
 });
```

## About

The FerruX Canvas is a tool developed while creating the FerruXengine, an attempt of 3D graphics engine I was trying to
make. I made this canvas to ease the use of the *Pixels* buffer used by the engine. As the tool proved useful for 
other possible future projects I was thinking of, and a couple of mutuals of mine thought that they could use it too, 
it was decided to extract it as its own library.