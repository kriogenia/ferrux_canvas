//! Throwable errors of the API
use std::error::Error;
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};

/// Errors than can occur using a **Canvas**
pub enum CanvasError {
	/// No GPU adapter has been found to run the pixel buffer
	AdapterNotFound,
	/// Error triggered during a render
	Rendering,
}

impl CanvasError {
	fn message(&self) -> &str {
		match self {
			Self::AdapterNotFound => "GPU adapter not found",
			Self::Rendering => "Rendering has failed",
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

#[cfg(test)]
mod tests {
	use crate::canvas::canvas_error::CanvasError;

	#[test]
	fn test_send() {
		fn assert_send<T: Send>() {}
		assert_send::<CanvasError>();
	}

	#[test]
	fn test_sync() {
		fn assert_sync<T: Sync>() {}
		assert_sync::<CanvasError>();
	}
}