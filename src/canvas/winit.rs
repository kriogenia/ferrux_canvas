//! Tools of the library to work with [winit]

use std::fmt::{Debug, Formatter};
use bresenham_zip::bresenham::BresenhamZipY;
use line_drawing::Bresenham;
use log::{error, info};
use pixels::{Pixels, SurfaceTexture};
use winit::window::Window;
use crate::canvas::canvas_error::CanvasError;
use crate::canvas::{Canvas, Point};
use crate::canvas::helpers::{as_signed, as_u32, calculate_intersection, sort_vectors};
use crate::color::*;

/// Canvas to use with a [winit::window::Window]
pub struct WinitCanvas {
	pixels: Pixels,
	canvas: Vec<Vec<Color>>,
	width: u32,
	height: u32,
}

impl WinitCanvas {

	/// Returns a new Winit canvas
	///
	/// # Arguments
	/// * `window` - Borrowed [Window] to draw on
	///
	/// # Errors
	/// If no adapter for the GPU is found a [CanvasError::AdapterNotFound] is thrown
	///
	/// # Example
	/// ```no_run
	/// # use std::error::Error;
	/// # use ferrux_canvas::canvas::Canvas;
	/// #
	/// # fn main() -> Result<(), Box<dyn Error>> {
	/// let mut events_loop = winit::event_loop::EventLoop::new();
	/// let window = winit::window::Window::new(&events_loop)?;
	/// let mut canvas = ferrux_canvas::canvas::winit::WinitCanvas::new(&window)?;
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
			canvas: vec![vec![palette::BLACK; height as usize]; width as usize],
			width,
			height,
		})
	}

	/// Draws an horizontal line between two points
	fn draw_horizontal_line(&mut self, start: Point, end: Point, color: Color) {
		let y = start.1;
		for x in if start.0 < end.0 { start.0..=end.0 } else { end.0..=start.0 } {
			self.draw_pixel(x, y, color.clone());
		}
	}

	/// Draws a vertical line between two points
	fn draw_vertical_line(&mut self, start: Point, end: Point, color: Color) {
		let x = start.0;
		for y in if start.1 < end.1 { start.1..=end.1 } else { end.1..=start.1} {
			self.draw_pixel(x, y, color.clone());
		}
	}

	/// Draws a diagonal line between two points using Bresenham's algorithm
	fn draw_diagonal_line(&mut self, start: Point, end: Point, color: Color) {
		for (x, y) in Bresenham::new(as_signed(start),as_signed(end)) {
			self.draw_pixel(x as u32, y as u32, color.clone());
		}
	}

	/// Fills the flat triangle (a triangle were two points share the same height) made with the three
	/// passed points using Bresenham
	fn fill_flat_triangle(&mut self, peak: Point, side_a: Point, side_b: Point, color: Color) {
		let bresenham = BresenhamZipY::new(as_signed(peak), as_signed(side_a), as_signed(side_b));
		for (left, right) in bresenham.unwrap() {
			self.draw_line(as_u32(left), as_u32(right), color.clone());
		}
	}

}

impl Canvas for WinitCanvas {

	fn width(&self) -> u32 {
		self.width
	}

	fn height(&self) -> u32 {
		self.height
	}

	/// Renders the current canvas in the screen
	///
	/// # Example
	/// The best way to use it is inside a new event loop thread when the redraw requested is
	/// triggered (something that can be done with the [Window::request_redraw]
	///
	/// ```no_run
	/// # use winit::event::Event;
	/// # use ferrux_canvas::canvas::Canvas;
	/// #
	/// # let mut event_loop = winit::event_loop::EventLoop::new();
	/// # let window = winit::window::Window::new(&event_loop).unwrap();
	/// # let mut canvas = ferrux_canvas::canvas::winit::WinitCanvas::new(&window).unwrap();
	/// event_loop.run(move |event, _, _| {
	///   match event {
	///     Event::MainEventsCleared => {
	///       window.request_redraw();
	///     }
	///     Event::RedrawRequested(_) => {
	///       canvas.render().unwrap();
	///     }
	///     _ => (),
	///  }
	/// });
	/// ```
	///
	/// # Errors
	/// [CanvasError::Rendering] if something goes wrong loading the current texture
	///
	fn render(&mut self) -> Result<(), CanvasError> {
		for (i, pixel) in self.pixels.get_frame().chunks_exact_mut(4).enumerate() {
			pixel.copy_from_slice(&self.canvas[i % self.width as usize][i / self.width as usize].as_u8());
		}

		self.pixels.render().map_err(|e| {
			error!("pixels.render() failed: {:?}", e);
			CanvasError::Rendering
		})
	}

	fn draw_pixel(&mut self, x: u32, y: u32, color: Color) {
		if x < self.width && y < self.height {
			self.canvas[x as usize][y as usize] = color;
		}
	}

	fn draw_line(&mut self, start: Point, end: Point, color: Color) {
		match start {
			(x, _) if x == end.0 => self.draw_vertical_line(start, end, color),
			(_, y) if y == end.1 => self.draw_horizontal_line(start, end, color),
			_ => self.draw_diagonal_line(start, end, color)
		}
	}

	fn draw_triangle(&mut self, point_a: Point, point_b: Point, point_c: Point, color: Color) {
		self.draw_line(point_a, point_b, color.clone());
		self.draw_line(point_b, point_c, color.clone());
		self.draw_line(point_c, point_a, color);
	}

	fn fill_triangle(&mut self, p1: Point, p2: Point, p3: Point, color: Color) {
		let (p1, p2, p3) = sort_vectors(p1, p2, p3);
		match p2 {
			(_, y) if y == p1.1 => self.fill_flat_triangle(p3, p1, p2, color),
			(_, y) if y == p3.1 => self.fill_flat_triangle(p1, p2, p3, color),
			_ => {
				let p4 = calculate_intersection(p3, p2, p1);
				self.fill_flat_triangle(p1, p2, p4, color.clone());
				self.fill_flat_triangle(p3, p2, p4, color);
			}
		}
	}

	fn clear_frame(&mut self) -> Result<(), CanvasError> {
		for pixel in self.pixels.get_frame().chunks_exact_mut(4) {
			pixel.copy_from_slice(&palette::BLACK.as_u8());
		}

		self.pixels.render().map_err(|e| {
			error!("pixels.render() failed: {:?}", e);
			CanvasError::Rendering
		})
	}

	fn reset_frame(&mut self) {
		self.canvas = vec![vec![palette::BLACK; self.height as usize]; self.width as usize];
	}

	fn resize(&mut self, width: u32, height: u32) {
		self.width = width;
		self.height = height;
		self.reset_frame();
		self.pixels.resize_surface(width, height);
	}

}

impl Debug for WinitCanvas {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(f, "[WinitCanvas]\
		Width: {},\
		Height: {},\
		Current canvas: {:?}", self.width, self.height, self.canvas)
	}
}