use log::{error, info};
use pixels::{Pixels, SurfaceTexture};
use winit::window::Window;
use crate::canvas::canvas_error::CanvasError;

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
	/// let mut events_loop = winit::event_loop::EventLoop::new();
	/// let window = winit::window::Window::new(&events_loop).unwrap();
	/// let mut canvas = ferrux_canvas::canvas::Canvas::new(&window).unwrap();
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
			canvas: vec![vec![Pixel::Blank; height as usize]; width as usize],
			width,
			height,
		})
	}

	/// Width of the screen, matches the window width
	///
	/// ```no_run
	/// let window = winit::window::Window::new(&winit::event_loop::EventLoop::new()).unwrap();
	/// let mut canvas = ferrux_canvas::canvas::Canvas::new(&window).unwrap();
	///
	/// assert_eq!(window.inner_size().width, canvas.width());
	/// ```
	///
	pub fn width(&self) -> u32 {
		self.width
	}

	/// Height of the screen, matches the window height
	///
	/// ```no_run
	/// let window = winit::window::Window::new(&winit::event_loop::EventLoop::new()).unwrap();
	/// let mut canvas = ferrux_canvas::canvas::Canvas::new(&window).unwrap();
	///
	/// assert_eq!(window.inner_size().height, canvas.height());
	/// ```
	///
	pub fn height(&self) -> u32 {
		self.height
	}

	/// Renders the current canvas in the screen and clears it to
	///
	/// # Errors
	/// [CanvasError::Rendering] if something goes wrong
	///
	pub fn render(&mut self) -> Result<(), CanvasError> {
		for (i, pixel) in self.pixels.get_frame().chunks_exact_mut(4).enumerate() {
			match self.canvas[i % self.width as usize][i / self.width as usize] {
				Pixel::White => pixel.copy_from_slice(&[0xff, 0xff, 0xff, 0xff]),
				Pixel::Blank => pixel.copy_from_slice(&[0x00, 0x00, 0x00, 0x00])
			}
		}

		//self.clear();

		self.pixels.render().map_err(|e| {
			error!("pixels.render() failed: {:?}", e);
			CanvasError::Rendering
		})
	}

	/// Draws a single pixel on the buffer, ready to be printed in the next [Canvas::render] call.
	pub fn draw_pixel(&mut self, x: u32, y: u32) {
		if x < self.width && y < self.height {
			self.canvas[x as usize][y as usize] = Pixel::White;
		}
	}

}

#[derive(Clone, Copy)]
enum Pixel {
	Blank,
	White,
}
