use regex::Regex;
use crate::color::color_error::ColorError;
use crate::color::color_error::ColorError::InvalidSyntax;

/// Representation of a color to use in the canvas based on RGBA
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

	pub fn as_u8(&self) -> [u8; 4] {
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
	fn valid_rgba_parsing() {
		let color = Color::from_rgba("0a2B3c4d");
		let expected = Color { r: 10, g: 43, b: 60, a: 77 };
		assert_eq!(color.unwrap(), expected);
	}

	#[test]
	fn invalid_rgba_parsing() {
		// Short
		assert!(Color::from_rgba("0a2B3c4").is_err());
		// Wrong char
		assert!(Color::from_rgba("0Z2B3c4d").is_err());
	}

}