use crate::canvas::Point;

/// Receives three points and returns them sorted by Y value.
/// This is a method to ease the finding of the middle vector and both peaks when filling a triangle
pub fn sort_vectors(p1: Point, p2: Point, p3: Point) -> (Point, Point, Point) {
	let mut points = [p1, p2, p3];
	points.sort_by_key(|&(x, y)| (!y, x));
	(points[0], points[1], points[2])
}

pub fn as_isize(point: Point) -> (isize, isize) {
	(point.0 as isize, point.1 as isize)
}

pub fn as_u32(point: (isize, isize)) -> Point {
	(point.0 as u32, point.1 as u32)
}

#[cfg(test)]
mod tests {
	use super::sort_vectors;

	#[test]
	fn sort_vectors_test() {
		assert_eq!(((10,10), (5,5), (0,0)), sort_vectors((10,10), (5,5), (0,0)));
		assert_eq!(((5,10), (10,5), (0,0)), sort_vectors((10,5), (5,10), (0,0)));
		assert_eq!(((0,10), (10,5), (5,0)), sort_vectors((5,0), (10,5), (0,10)));
	}

}