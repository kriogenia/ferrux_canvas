//! Color tools to draw on the canvas
pub use rgba_color::Color;
pub use color_builder::ColorBuilder;

mod rgba_color;
mod color_error;
pub mod palette;
mod color_builder;