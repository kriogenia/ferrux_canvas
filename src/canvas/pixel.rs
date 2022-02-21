#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Pixel {
	Background,
	Foreground,
}

impl Pixel {
	/// Returns the color stored in the pixel
	pub fn color(&self) -> &[u8] {
		match self {
			Pixel::Background => &[0x00, 0x00, 0x00, 0x00],
			Pixel::Foreground => &[0xff, 0xff, 0xff, 0xff],
		}
	}
}