pub use canvas::Canvas;

mod canvas;
pub mod canvas_error;

/// Tuple of two **u32** values representing a drawable point
///
/// ```rust
/// let point = (100, 200); // Represents the point (x: 100, y: 200) of the screen
/// ```
pub type Point = (u32, u32);