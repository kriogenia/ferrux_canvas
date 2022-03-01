use crate::canvas::Point;

pub fn sort_vectors(p1: Point, p2: Point, p3: Point) -> (Point, Point, Point) {
	let mut points = [p1, p2, p3];
	points.sort_by_key(|&(x, y)| (!y, x));
	(points[0], points[1], points[2])
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