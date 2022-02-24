use crate::color::Color;

pub struct Palette<'a> {
	r: &'a str,
	g: &'a str,
	b: &'a str,
	a: &'a str
}

impl<'a> Palette<'a> {

	pub fn WHITE() -> Color {
		Color {
			r: 255,
			g: 255,
			b: 255,
			a: 255
		}
	}

	pub fn BLACK() -> Color {
		Color {
			r: 0,
			g: 0,
			b: 0,
			a: 255
		}
	}

}