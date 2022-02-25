use crate::color::Color;

/// Builder to construct colors specifying the value of the four scales: R, G, B, A
///
/// # Usage
/// The following example mimics the creation of the css `coral` color (`#ff7f50`).
/// ```rust
/// # use ferrux_canvas::color::{Color, ColorBuilder};
/// let coral = ColorBuilder::new().with_red(255).with_green(127).with_blue(80).build();
/// assert_eq!(coral, Color::from_rgba("ff7f50ff").unwrap());
/// ```
///
pub struct ColorBuilder {
	r: u8,
	g: u8,
	b: u8,
	a: u8
}

const DEFAULT_VALUE: u8 = u8::MAX;

impl ColorBuilder {

	/// Creates a new ColorBuilder
	pub fn new() -> ColorBuilder {
		ColorBuilder {
			r: DEFAULT_VALUE,
			g: DEFAULT_VALUE,
			b: DEFAULT_VALUE,
			a: DEFAULT_VALUE,
		}
	}

	/// Sets the RED scale of the color to build
	pub fn with_red(mut self, r: u8) -> ColorBuilder {
		self.r = r;
		self
	}

	/// Sets the BLUE scale of the color to build
	pub fn with_blue(mut self, b: u8) -> ColorBuilder {
		self.b = b;
		self
	}

	/// Sets the GREEN scale of the color to build
	pub fn with_green(mut self, g: u8) -> ColorBuilder {
		self.g = g;
		self
	}

	/// Sets the ALPHA scale of the color to build
	pub fn with_alpha(mut self, a: u8) -> ColorBuilder {
		self.a = a;
		self
	}

	/// Builds the color with the specified scales
	pub fn build(&self) -> Color {
		Color {
			r: self.r,
			g: self.g,
			b: self.b,
			a: self.a
		}
	}

}