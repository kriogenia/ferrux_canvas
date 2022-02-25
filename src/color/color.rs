use regex::Regex;
use crate::color::color_error::ColorError;
use crate::color::color_error::ColorError::InvalidSyntax;

/// Representation of a color to use in the canvas based on RGBA.
/// It should be passed to the drawing operations to specify the color to use.
/// It can be constructed with the [`from_rgba`] function passing to hexadecimal string representation
/// or using the [`ColorBuilder`]. Some useful predefined colors are also available at [`palette`]
///
/// # Example
/// The following example draws a triangle using a line of each primary RGB color.
/// ```no_run
/// # use std::error::Error;
/// # use ferrux_canvas::color::*;
/// # use ferrux_canvas::canvas::Canvas;
/// # fn main() -> Result<(), Box<dyn Error>> {
/// # let mut events_loop = winit::event_loop::EventLoop::new();
/// # let window = winit::window::Window::new(&events_loop)?;
/// # let mut canvas = ferrux_canvas::canvas::winit::WinitCanvas::new(&window)?;
/// let red = palette::RED;
/// let green = Color::from_rgba("00ff00ff")?;
/// let blue = ColorBuilder::new().with_red(0).with_green(0).with_blue(255).build();
///
/// canvas.draw_line((100, 100), (100, 200));
/// canvas.draw_line((100, 200), (150, 150));
/// canvas.draw_line((150, 150), (100, 100));
/// # Ok(()) }
/// ```
///
/// [`from_rgba`]: super::Color::from_rgba
/// [`ColorBuilder`]: super::ColorBuilder
/// [`palette`]: super::palette
#[derive(Debug, PartialEq)]
pub struct Color {
	pub r: u8,
	pub g: u8,
	pub b: u8,
	pub a: u8
}

impl Color {

	/// Create a color from a RGBA hexadecimal representation. The syntax should be of type 'hhhhhhhh'
	/// without the # (intended to be supported on future versions).
	///
	/// # Example
	/// ```rust
	/// # use ferrux_canvas::color::Color;
	/// let color = Color::from_rgba("0a2B3c4d");
	/// let expected = Color { r: 10, g: 43, b: 60, a: 77 };
	/// assert_eq!(color.unwrap(), expected);
	/// ```
	///
	pub fn from_rgba(rgba: &str) -> Result<Color, ColorError> {
		match Regex::new(r"([a-fA-F\d]{2})([a-fA-F\d]{2})([a-fA-F\d]{2})([a-fA-F\d]{2})").unwrap()
			.captures(rgba) {
			Some(caps) => {
				if caps.len() != 5 {
					Err(InvalidSyntax("hhhhhhhh"))
				} else {
					let r = u8::from_str_radix(&caps[1], 16).unwrap();
					let g = u8::from_str_radix(&caps[2], 16).unwrap();
					let b = u8::from_str_radix(&caps[3], 16).unwrap();
					let a = u8::from_str_radix(&caps[4], 16).unwrap();
					Ok(Color { r, g, b, a })
				}
			}
			None => Err(InvalidSyntax("hhhhhhhh"))
		}
	}

	/// Returns the u8 array representation of the color to be passed to the pixels buffer
	pub(crate) fn as_u8(&self) -> [u8; 4] {
		[self.r, self.g, self.b, self.a]
	}

}

impl Clone for Color {
	fn clone(&self) -> Self {
		Self {
			r: self.r,
			g: self.g,
			b: self.b,
			a: self.a
		}
	}
}

#[cfg(test)]
mod tests {
	use crate::color::Color;

	#[test]
	fn invalid_rgba_parsing() {
		// Short
		assert!(Color::from_rgba("0a2B3c4").is_err());
		// Wrong char
		assert!(Color::from_rgba("0Z2B3c4d").is_err());
	}

}