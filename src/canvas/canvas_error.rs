use std::error::Error;
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};

/// Errors than can occur using a [Canvas]
pub enum CanvasError {
	/// No GPU adapter has been found to run the pixel buffer
	AdapterNotFound,
}

impl CanvasError {
	fn message(&self) -> &str {
		match self {
			Self::AdapterNotFound => "GPU adapter not found",
		}
	}
}

impl Error for CanvasError {}

impl Debug for CanvasError {
	fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
		write!(f, "{}", self.message())
	}
}

impl Display for CanvasError {
	fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
		write!(f, "{}", self.message())
	}
}