use log::info;
use pixels::{Pixels, SurfaceTexture};
use winit::window::Window;
use crate::canvas::canvas_error::CanvasError;

/// Canvas to manage what is drawn in the screen
pub struct Canvas {
	pixels: Pixels,
	canvas: Vec<Vec<Pixel>>,
	width: usize,
	height: usize,
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
	pub fn new(window: &Window) -> Result<Self, CanvasError> {
		info!("Starting FerruX Canvas");
		
		let window_size = window.inner_size();
		let width = window_size.width as usize;
		let height = window_size.height as usize;
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

	/// Width of the screen
	pub fn width(&self) -> usize {
		self.width
	}

	/// Height of the screen
	pub fn height(&self) -> usize {
		self.height
	}
	
}

#[derive(Clone, Copy)]
enum Pixel {
	Blank,
	White,
}
