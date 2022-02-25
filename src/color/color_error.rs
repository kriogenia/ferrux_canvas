//! Throwable errors using the color API
use std::error::Error;
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};

/// Errors than can occur using [`Color`]
///
/// [`Color`]: crate::color::Color
pub enum ColorError<'a> {
	/// The syntax of the color is invalid
	InvalidSyntax(&'a str),
}

impl<'a> ColorError<'a> {
	fn message(&self) -> String {
		match self {
			Self::InvalidSyntax(syntax) => "The color syntax is invalid. It should be. ".to_owned() + *syntax,
		}
	}
}

impl<'a> Error for ColorError<'a> {}

impl<'a> Debug for ColorError<'a> {
	fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
		write!(f, "{}", self.message())
	}
}

impl<'a> Display for ColorError<'a> {
	fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
		write!(f, "{}", self.message())
	}
}

#[cfg(test)]
mod tests {
	use crate::color::color_error::ColorError;

	#[test]
	fn test_send() {
		fn assert_send<T: Send>() {}
		assert_send::<ColorError>();
	}

	#[test]
	fn test_sync() {
		fn assert_sync<T: Sync>() {}
		assert_sync::<ColorError>();
	}
}