//! The Canvas trait and all its implementations

pub use canvas_trait::Canvas;

pub mod winit;
pub mod canvas_error;
mod canvas_trait;
mod helpers;

/// Tuple of two **u32** values representing a drawable point
///
/// ```rust
/// let point = (100, 200); // Represents the point (x: 100, y: 200) of the screen
/// ```
pub type Point = (u32, u32);