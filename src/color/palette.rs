//! Collection of basic constants colors to use in the canvas
use crate::color::Color;

/// White color: #ffffff
pub const WHITE: Color = Color {
	r: u8::MAX,
	g: u8::MAX,
	b: u8::MAX,
	a: u8::MAX,
};

/// Black color: #000000
pub const BLACK: Color = Color {
	r: u8::MIN,
	g: u8::MIN,
	b: u8::MIN,
	a: u8::MAX,
};

/// Red color: #ff0000
pub const RED: Color = Color {
	r: u8::MAX,
	g: u8::MIN,
	b: u8::MIN,
	a: u8::MAX
};

/// Green color: #00ff00
pub const GREEN: Color = Color {
	r: u8::MIN,
	g: u8::MAX,
	b: u8::MIN,
	a: u8::MAX,
};

/// Blue color: #0000ff
pub const BLUE: Color = Color {
	r: u8::MIN,
	g: u8::MIN,
	b: u8::MAX,
	a: u8::MAX,
};