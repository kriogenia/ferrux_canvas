use crate::canvas::canvas_error::CanvasError;
use crate::canvas::Point;
use crate::color::Color;

/// Entity managing the screen rendering and providing the tools to perform the drawing of the
/// the figures and invoke the frame rendering.
pub trait Canvas {

	/// Width of the canvas screen
	///
	/// ```no_run
	/// # use std::error::Error;
	/// # use ferrux_canvas::canvas::Canvas;
	/// #
	/// # fn main() -> Result<(), Box<dyn Error>> {
	/// # let window = winit::window::Window::new(&winit::event_loop::EventLoop::new())?;
	/// let mut canvas = ferrux_canvas::canvas::winit::WinitCanvas::new(&window)?;
	/// assert_eq!(window.inner_size().width, canvas.width());
	/// # Ok(())}
	/// ```
	///
	fn width(&self) -> u32;

	/// Height of the canvas screen
	///
	/// ```no_run
	/// # use std::error::Error;
	/// # use ferrux_canvas::canvas::Canvas;
	/// #
	/// # fn main() -> Result<(), Box<dyn Error>> {
	/// # let window = winit::window::Window::new(&winit::event_loop::EventLoop::new())?;
	/// let mut canvas = ferrux_canvas::canvas::winit::WinitCanvas::new(&window)?;
	/// assert_eq!(window.inner_size().height, canvas.height());
	/// # Ok(())}
	/// ```
	///
	fn height(&self) -> u32;

	/// Renders the current frame in the screen.
	/// See implementations like [super::winit::WinitCanvas::render] to get more information on each
	/// particular use case.
	///
	fn render(&mut self) -> Result<(), CanvasError>;

	/// Draws a single pixel on the buffer, ready to be printed in the next [Canvas::render] call.
	///
	/// # Example
	/// The pixel (100, 100) will be drawn blue on the screen with the next render call.
	/// ```no_run
	/// # use ferrux_canvas::canvas::Canvas;
	/// # use ferrux_canvas::color::palette;
	/// # let window = winit::window::Window::new(&winit::event_loop::EventLoop::new()).unwrap();
	/// # let mut canvas = ferrux_canvas::canvas::winit::WinitCanvas::new(&window).unwrap();
	/// canvas.draw_pixel(100, 100, palette::BLUE);
	/// ```
	///
	fn draw_pixel(&mut self, x: u32, y: u32, color: Color);

	/// Draws a line between the two specified points in the canvas
	///
	/// # Arguments
	/// * `start` - Starting point
	/// * `end` - Ending point
	///
	/// # Example
	/// The line between the pixels (100, 100) and (200, 200) will be drawn on the screen in red with 
	/// the next render call.
	/// ```no_run
	/// # use ferrux_canvas::canvas::Canvas;
	/// # use ferrux_canvas::color::palette;
	/// # let window = winit::window::Window::new(&winit::event_loop::EventLoop::new()).unwrap();
	/// # let mut canvas = ferrux_canvas::canvas::winit::WinitCanvas::new(&window).unwrap();
	/// canvas.draw_line((100, 100), (200, 200), palette::RED);
	/// ```
	///
	fn draw_line(&mut self, start: Point, end: Point, color: Color);

	/// Draws the three lines compounding a triangle in the canvas
	///
	/// # Arguments
	/// `triangle` - 2D Triangle to draw
	///
	/// # Example
	/// The triangle between the pixels (100, 100), (100, 150) and (150, 100) will be drawn green on 
	/// the screen with the next render call.
	/// ```no_run
	/// # use ferrux_canvas::canvas::Canvas;
	/// # use ferrux_canvas::color::palette;
	/// # let window = winit::window::Window::new(&winit::event_loop::EventLoop::new()).unwrap();
	/// # let mut canvas = ferrux_canvas::canvas::winit::WinitCanvas::new(&window).unwrap();
	/// canvas.draw_triangle((100, 100), (100, 150), (150, 100), palette::GREEN);
	/// ```
	///
	fn draw_triangle(&mut self, point_a: Point, point_b: Point, point_c: Point, color: Color);

	/// Renders an empty frame. It mimics a call to [Canvas::render] after a [Canvas::reset_frame] but
	/// it doesn't clear the buffer. Allowing to clear the screen without losing the current drawn
	/// image
	///
	/// # Example
	/// ```no_run
	/// # use std::time::Duration;
	/// # use ferrux_canvas::canvas::Canvas;
	/// # use ferrux_canvas::color::palette;
	/// # let window = winit::window::Window::new(&winit::event_loop::EventLoop::new()).unwrap();
	/// # let mut canvas = ferrux_canvas::canvas::winit::WinitCanvas::new(&window).unwrap();
	/// canvas.draw_pixel(150, 100, palette::WHITE);
	/// canvas.render();      // This would render the pixel on the screen
	/// std::thread::sleep(Duration::new(2, 0));
	/// canvas.clear_frame(); // This would render a blank screen but the buffer is kept
	/// std::thread::sleep(Duration::new(2, 0));
	/// canvas.render();      // The pixel would be rendered again with this call
	/// ```
	///
	fn clear_frame(&mut self) -> Result<(), CanvasError>;

	/// Clears the current buffer, allowing to draw a completely new frame without the previous data
	///
	/// # Example
	/// ```no_run
	/// # use ferrux_canvas::canvas::Canvas;
	/// # use ferrux_canvas::color::palette;
	/// # let window = winit::window::Window::new(&winit::event_loop::EventLoop::new()).unwrap();
	/// # let mut canvas = ferrux_canvas::canvas::winit::WinitCanvas::new(&window).unwrap();
	/// canvas.draw_pixel(150, 100, palette::WHITE);
	/// canvas.render();    // This would render the pixel on the screen
	/// std::thread::sleep(std::time::Duration::new(2, 0));
	/// canvas.reset_frame();
	/// canvas.render();    // This would render an empty frame
	/// ```
	fn reset_frame(&mut self);

	/// Resizes the canvas to a new size. The current frame is lost doing it.
	///
	/// # Arguments
	/// * `size` - New size for the canvas
	///
	/// # Example
	/// ```no_run
	/// # use ferrux_canvas::canvas::Canvas;
	/// # let window = winit::window::Window::new(&winit::event_loop::EventLoop::new()).unwrap();
	/// # let mut canvas = ferrux_canvas::canvas::winit::WinitCanvas::new(&window).unwrap();
	/// canvas.resize(640, 480);
	/// assert_eq!(640, canvas.width());
	/// assert_eq!(480, canvas.height());
	/// ```
	///
	fn resize(&mut self, width: u32, height: u32);

}