pub trait Point<T> {

	fn x(&self) -> T;
	fn y(&self) -> T;
	fn z(&self) -> T;

}

/// Tuple of two **u32** values representing a two dimensional drawable point
///
/// ```rust
/// let point = (100, 200); // Represents the point (x: 100, y: 200) of the screen
/// ```
pub struct Point2(u32, u32);

/// Tuple of three **u32** values representing a three dimensional drawable point
///
/// ```rust
/// let point = (10, 25, 15); // Represents the point (x: 10, y: 25, y: 15) of the screen
/// ```
pub struct Point3(u32, u32, u32);

impl Point<u32> for Point2 {
	fn x(&self) -> u32 {
		self.0
	}

	fn y(&self) -> u32 {
		self.1
	}

	fn z(&self) -> u32 {
		0
	}
}

impl Point<u32> for Point2 {
	fn x(&self) -> u32 {
		self.0
	}

	fn y(&self) -> u32 {
		self.1
	}

	fn z(&self) -> u32 {
		self.2
	}
}

