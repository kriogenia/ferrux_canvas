pub use canvas::Canvas;

pub mod winit;
pub mod canvas_error;
mod pixel;
mod canvas;

/// Tuple of two **u32** values representing a drawable point
///
/// ```rust
/// let point = (100, 200); // Represents the point (x: 100, y: 200) of the screen
/// ```
pub type Point = (u32, u32);