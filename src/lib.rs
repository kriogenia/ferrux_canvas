pub mod canvas;

/// Tuple of two **u32** values representing a drawable point
///
/// ```rust
/// let point = (100, 200); // Represents the point (x: 100, y: 200) of the screen
/// ```
pub type Point = (u32, u32);

/// Tuple of three [Point] representing a drawable triangle
///
/// ```rust
/// let triangle = ((100, 100), (150, 200), (200, 100)); // Represents a triangle joining those three points
/// ```
pub type Triangle = (Point, Point, Point);