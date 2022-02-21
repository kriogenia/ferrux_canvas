use bresenham::Bresenham;
use log::{error, info};
use pixels::{Pixels, SurfaceTexture};
use winit::window::Window;
use crate::canvas::canvas_error::CanvasError;
use crate::canvas::pixel::Pixel;
use crate::canvas::Point;

/// Canvas to manage what is drawn in the screen
pub struct Canvas {
	pixels: Pixels,
	canvas: Vec<Vec<Pixel>>,
	width: u32,
	height: u32,
}

impl Canvas {

	/// Returns a new canvas
	///
	/// # Arguments
	/// * `window` - Borrowed winit [Window] to draw on
	///
	/// # Errors
	/// If no adapter for the GPU is found a [CanvasError::AdapterNotFound] is thrown
	///
	/// # Example
	/// ```no_run
	/// # use std::error::Error;
	/// #
	/// # fn main() -> Result<(), Box<dyn Error>> {
	/// let mut events_loop = winit::event_loop::EventLoop::new();
	/// let window = winit::window::Window::new(&events_loop)?;
	/// let mut canvas = ferrux_canvas::canvas::Canvas::new(&window)?;
	/// # Ok(())}
	/// ```
	///
	pub fn new(window: &Window) -> Result<Self, CanvasError> {
		info!("Starting FerruX Canvas");

		let window_size = window.inner_size();
		let width = window_size.width;
		let height = window_size.height;
		info!("[Ferrux Canvas] Width: {}. Height: {}", &width, &height);

		info!("[Ferrux Canvas] Creating pixel buffer");
		let pixels = {
			let surface_texture =
				SurfaceTexture::new(window_size.width, window_size.height, &window);
			Pixels::new(window_size.width, window_size.height, surface_texture)
				.map_err(|_| CanvasError::AdapterNotFound)?
		};

		Ok(Self {
			pixels,
			canvas: vec![vec![Pixel::Background; height as usize]; width as usize],
			width,
			height,
		})
	}

	/// Width of the screen, matches the window width
	///
	/// ```no_run
	/// # use std::error::Error;
	/// #
	/// # fn main() -> Result<(), Box<dyn Error>> {
	/// # let window = winit::window::Window::new(&winit::event_loop::EventLoop::new())?;
	/// # let mut canvas = ferrux_canvas::canvas::Canvas::new(&window)?;
	/// assert_eq!(window.inner_size().width, canvas.width());
	/// # Ok(())}
	/// ```
	///
	pub fn width(&self) -> u32 {
		self.width
	}

	/// Height of the screen, matches the window height
	///
	/// ```no_run
	/// # use std::error::Error;
	/// #
	/// # fn main() -> Result<(), Box<dyn Error>> {
	/// # let window = winit::window::Window::new(&winit::event_loop::EventLoop::new())?;
	/// # let mut canvas = ferrux_canvas::canvas::Canvas::new(&window)?;
	/// assert_eq!(window.inner_size().height, canvas.height());
	/// # Ok(())}
	/// ```
	///
	pub fn height(&self) -> u32 {
		self.height
	}

	/// Renders the current canvas in the screen and clears it to
	///
	/// # Example
	/// The best way to use it is inside a new event loop thread when the redraw requested is
	/// triggered (something that can be done with the [Window::request_redraw]
	///
	/// ```no_run
	/// # use winit::event::Event;
	///
	/// # let mut event_loop = winit::event_loop::EventLoop::new();
	/// # let window = winit::window::Window::new(&event_loop).unwrap();
	/// # let mut canvas = ferrux_canvas::canvas::Canvas::new(&window).unwrap();
	/// event_loop.run(move |event, _, _| {
	/// 		match event {
	/// 			Event::MainEventsCleared => {
	/// 				window.request_redraw();
	/// 			}
	/// 			Event::RedrawRequested(_) => {
	/// 				canvas.render().unwrap();
	/// 			}
	/// 			_ => (),
	/// 		}
 	/// });
	/// ```
	///
	/// # Errors
	/// [CanvasError::Rendering] if something goes wrong loading the current texture
	///
	pub fn render(&mut self) -> Result<(), CanvasError> {
		for (i, pixel) in self.pixels.get_frame().chunks_exact_mut(4).enumerate() {
			pixel.copy_from_slice(self.canvas[i % self.width as usize][i / self.width as usize].color());
		}

		self.pixels.render().map_err(|e| {
			error!("pixels.render() failed: {:?}", e);
			CanvasError::Rendering
		})
	}

	/// Draws a single pixel on the buffer, ready to be printed in the next [Canvas::render] call.
	///
	/// # Example
	/// The pixel (100, 100) will be drawn on the screen with the next render call.
	/// ```no_run
	/// # let window = winit::window::Window::new(&winit::event_loop::EventLoop::new()).unwrap();
	/// # let mut canvas = ferrux_canvas::canvas::Canvas::new(&window).unwrap();
	/// canvas.draw_pixel(100, 100);
	/// ```
	///
	pub fn draw_pixel(&mut self, x: u32, y: u32) {
		if x < self.width && y < self.height {
			self.canvas[x as usize][y as usize] = Pixel::Foreground;
		}
	}

	/// Draws a line between the two specified points in the canvas
	///
	/// # Arguments
	/// * `start` - Starting point
	/// * `end` - Ending point
	///
	/// # Example
	/// The line between the pixels (100, 100) and (200, 200) will be drawn on the screen with the next
	/// render call.
	/// ```no_run
	/// # let window = winit::window::Window::new(&winit::event_loop::EventLoop::new()).unwrap();
	/// # let mut canvas = ferrux_canvas::canvas::Canvas::new(&window).unwrap();
	/// canvas.draw_line((100, 100), (200, 200));
	/// ```
	///
	pub fn draw_line(&mut self, start: Point, end: Point) {
		for (x, y) in Bresenham::new(
			(start.0 as isize, start.1 as isize), (end.0 as isize, end.1 as isize)) {
			self.draw_pixel(x as u32, y as u32);
		}
	}

	/// Draws the three lines compounding a triangle in the canvas
	///
	/// # Arguments
	/// `triangle` - 2D Triangle to draw
	///
	/// # Example
	/// The triangle between the pixels (100, 100), (100, 150) and (150, 100) will be drawn on the
	/// screen with the next render call.
	/// ```no_run
	/// # let window = winit::window::Window::new(&winit::event_loop::EventLoop::new()).unwrap();
	/// # let mut canvas = ferrux_canvas::canvas::Canvas::new(&window).unwrap();
	/// canvas.draw_triangle((100, 100), (100, 150), (150, 100));
	/// ```
	///
	pub fn draw_triangle(&mut self, point_a: Point, point_b: Point, point_c: Point) {
		self.draw_line(point_a, point_b);
		self.draw_line(point_b, point_c);
		self.draw_line(point_c, point_a);
	}

	/// Renders an empty frame. It mimics a call to [Canvas::render] after a [Canvas::reset_frame] but
	/// it doesn't clear the buffer. Allowing to clear the screen without losing the current drawn
	/// image
	///
	/// # Example
	/// ```no_run
	/// # use std::time::Duration;
	/// # let window = winit::window::Window::new(&winit::event_loop::EventLoop::new()).unwrap();
	/// # let mut canvas = ferrux_canvas::canvas::Canvas::new(&window).unwrap();
	/// canvas.draw_pixel(150, 100);
	/// canvas.render();      // This would render the pixel on the screen
	/// std::thread::sleep(Duration::new(2, 0));
	/// canvas.clear_frame(); // This would render a blank screen but the buffer is kept
	/// std::thread::sleep(Duration::new(2, 0));
	/// canvas.render();      // The pixel would be rendered again with this call
	/// ```
	///
	pub fn clear_frame(&mut self) -> Result<(), CanvasError> {
		for pixel in self.pixels.get_frame().chunks_exact_mut(4) {
			pixel.copy_from_slice(Pixel::Background.color());
		}

		self.pixels.render().map_err(|e| {
			error!("pixels.render() failed: {:?}", e);
			CanvasError::Rendering
		})
	}

	/// Clears the current buffer, allowing to draw a completely new frame without the previous data
	///
	/// # Example
	/// ```no_run
	/// # let window = winit::window::Window::new(&winit::event_loop::EventLoop::new()).unwrap();
	/// # let mut canvas = ferrux_canvas::canvas::Canvas::new(&window).unwrap();
	/// canvas.draw_pixel(150, 100);
	/// canvas.render();    // This would render the pixel on the screen
	/// std::thread::sleep(std::time::Duration::new(2, 0));
	/// canvas.reset_frame();
	/// canvas.render();    // This would render an empty frame
	/// ```
	pub fn reset_frame(&mut self) {
		self.canvas = vec![vec![Pixel::Background; self.height as usize]; self.width as usize];
	}

}