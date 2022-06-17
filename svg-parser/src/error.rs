use std::fmt::Debug;

use crate::span::ErrorPos;

#[derive(Default)]
pub enum StreamError {
	UnexpectedEndOfStream,
	UnexpectedData(ErrorPos),

	#[default]
	InvalidValue,
	InvalidString(Vec<String>, ErrorPos),
	InvalidCharacterRef(ErrorPos),
	InvalidNumber(ErrorPos),
	InvalidXMLName(ErrorPos),
	InvalidQuote(ErrorPos),
}

pub type StreamResult<T> = Result<T, StreamError>;

impl std::fmt::Display for StreamError {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		match self {
			StreamError::UnexpectedEndOfStream => write!(f, "unexpected end of stream"),
			StreamError::UnexpectedData(position) => write!(f, "unexpected data at position {}", position),
			StreamError::InvalidValue => write!(f, "invalid value"),
			StreamError::InvalidString(ref strings, position) => {
				write!(f, "expected '{}' not '{}' at position {}", strings[1..].join("', '"), strings[0], position)
			}
			StreamError::InvalidNumber(position) => write!(f, "invalid number at position {}", position),
			StreamError::InvalidCharacterRef(position) => write!(f, "invalid character ref at position {}", position),
			StreamError::InvalidXMLName(position) => write!(f, "invalid xml name at position {}", position),
			StreamError::InvalidQuote(position) => write!(f, r#"expected quote (" or ') at position {}"#, position),
		}
	}
}
impl std::fmt::Debug for StreamError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		<Self as std::fmt::Display>::fmt(self, f)
	}
}
#[derive(Debug)]
pub enum SvgErrorType {
	InvalidElement,
	InvalidProcessor,
	InvalidAttribute,
	UnknownToken,
	UnknownNamespace,
	DuplicateAttribute,
	InvalidCloseElement,
	CssInvalidString,
}

pub struct SvgError(pub SvgErrorType, pub StreamError, pub ErrorPos);

pub type SvgResult<T> = Result<T, SvgError>;

impl std::fmt::Display for SvgError {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		write!(f, "{:?} at {}. Reason: {}", self.0, self.2, self.1)
	}
}
impl std::fmt::Debug for SvgError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		<Self as std::fmt::Display>::fmt(self, f)
	}
}
