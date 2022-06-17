use core::fmt;
use std::{
	fmt::Display,
	ops::{Deref, Range},
};

use crate::{
	error::{StreamError, SvgError, SvgErrorType, SvgResult},
	xml_parse::{Document, Namespace},
};

#[derive(Clone, Copy, Default, Eq)]
pub struct Span<'a> {
	pub text: &'a str,
	pub start: usize,
}

impl<'a> From<&'a str> for Span<'a> {
	fn from(text: &'a str) -> Self {
		Self { text, start: 0 }
	}
}
impl<'a> PartialEq for Span<'a> {
	fn eq(&self, other: &Self) -> bool {
		self.text == other.text
	}
}

impl<'a> Span<'a> {
	pub fn from_range(source: &'a str, start: usize, end: usize) -> Self {
		Self { text: &source[start..end], start }
	}
	pub fn end(&self) -> usize {
		self.start + self.text.len()
	}
	pub fn range(&self) -> Range<u32> {
		self.start as u32..self.end() as u32
	}
}

impl<'a> fmt::Debug for Span<'a> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "StrSpan({:?} {}..{})", self.text, self.start, self.end())
	}
}

impl<'a> fmt::Display for Span<'a> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}", self.text)
	}
}

impl<'a> Deref for Span<'a> {
	type Target = str;

	fn deref(&self) -> &Self::Target {
		self.text
	}
}

#[derive(Default)]
pub struct ErrorPos {
	line: usize,
	col: usize,
}
impl Display for ErrorPos {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "Ln {}, Col {}", self.line, self.col)
	}
}
impl ErrorPos {
	pub fn new(source: &str, byte: usize) -> Self {
		Self {
			line: source.as_bytes()[..byte].iter().filter(|&&b| b == b'\n').count() + 1,
			col: source.as_bytes()[..byte].iter().rev().take_while(|&&b| b != b'\n').count() + 1,
		}
	}
}
#[derive(Debug, Default, PartialEq)]
pub struct QName<'a> {
	pub prefix: Span<'a>,
	pub local: Span<'a>,
	pub span: Span<'a>,
}
#[derive(Debug, Default, PartialEq)]
pub struct ResolvedQName<'a> {
	pub prefix: Span<'a>,
	pub local: Span<'a>,
	pub resolved: Option<&'a str>,
}
impl<'a> ResolvedQName<'a> {
	pub fn from_qname(qname: &QName<'a>, document: &Document<'a>, allow_empty: bool) -> SvgResult<Self> {
		let resolved = if !qname.prefix.is_empty() || !allow_empty {
			document
				.namespaces
				.iter()
				.find(|namespace| namespace.prefix == (!qname.prefix.text.is_empty()).then(|| qname.prefix.text))
				.ok_or(SvgError(SvgErrorType::UnknownNamespace, StreamError::InvalidValue, document.create_err(qname.prefix.start)))
				.map_or_else(|e| if qname.prefix.is_empty() { Ok(None) } else { Err(e) }, |namespace| Ok(Some(namespace)))?
				.map(|namespace| namespace.uri)
		} else {
			None
		};
		Ok(Self {
			prefix: qname.prefix,
			local: qname.local,
			resolved,
		})
	}
}
impl<'a> std::fmt::Display for ResolvedQName<'a> {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		if self.prefix.text.is_empty() {
			write!(f, r#""{}""#, self.local)
		} else {
			write!(f, r#""{}:{}""#, self.prefix, self.local)
		}
	}
}
