use crate::checks::{ByteExt, XmlCharExt};
use crate::error::{StreamError, StreamResult, SvgError, SvgErrorType, SvgResult};
use crate::span::{ErrorPos, QName, Span};
use std::str::FromStr;

#[derive(Clone, PartialEq, Debug)]
pub struct SvgStream<'a> {
	source: &'a str,
	position: usize,
}
impl<'a> From<&'a str> for SvgStream<'a> {
	fn from(source: &'a str) -> Self {
		Self::new(source)
	}
}
impl<'a> SvgStream<'a> {
	pub fn new(source: &str) -> SvgStream {
		SvgStream { source, position: 0 }
	}

	pub fn byte_pos(&self) -> usize {
		self.position
	}

	pub fn create_err(&self, byte: usize) -> ErrorPos {
		ErrorPos::new(self.source, byte)
	}

	pub fn calculate_characters_to_byte(&self, byte: usize) -> usize {
		self.source.char_indices().position(|(position, _)| position >= byte).unwrap_or(self.source.len()) + 1
	}

	pub fn calculate_characters(&self) -> usize {
		self.calculate_characters_to_byte(self.position)
	}

	pub fn at_end(&self) -> bool {
		self.position >= self.source.len()
	}

	pub fn current_byte_unchecked(&self) -> u8 {
		self.source.as_bytes()[self.position]
	}

	pub fn chars(&self) -> std::str::Chars<'a> {
		self.source[self.position..].chars()
	}

	pub fn current_byte(&self) -> StreamResult<u8> {
		(!self.at_end()).then(|| self.current_byte_unchecked()).ok_or(StreamError::UnexpectedEndOfStream)
	}

	pub fn match_current(&self, a: u8) -> bool {
		(!self.at_end()).then(|| self.current_byte_unchecked() == a).unwrap_or(false)
	}

	pub fn peek(&self) -> StreamResult<u8> {
		if self.position + 1 >= self.source.len() {
			return Err(StreamError::UnexpectedEndOfStream);
		}

		Ok(self.source.as_bytes()[self.position + 1])
	}

	pub fn advance(&mut self, n: usize) {
		debug_assert!(self.position + n <= self.source.len());
		self.position += n;
	}

	pub fn jump_to_end(&mut self) {
		self.position = self.source.len();
	}

	pub fn skip_spaces(&mut self) {
		let mut is_comment = self.starts_with("<!--".as_bytes());
		while !self.at_end() && (self.current_byte_unchecked().is_whitespace() || is_comment) {
			self.advance(1);
			// Comments are treated as whitespace
			if (!is_comment && self.starts_with("<!--".as_bytes())) {
				is_comment = true;
			} else if (is_comment && self.starts_with("-->".as_bytes())) {
				is_comment = false;
				self.advance(3);
			}
		}
	}

	#[inline]
	pub fn starts_with(&self, source: &[u8]) -> bool {
		self.source.as_bytes()[self.position..].starts_with(source)
	}

	pub fn consume_byte(&mut self, byte: u8) -> StreamResult<()> {
		if self.at_end() {
			Err(StreamError::UnexpectedEndOfStream)
		} else if !self.match_current(byte) {
			let actual = char::from(self.current_byte_unchecked()).into();
			let expected = char::from(byte).into();
			return Err(StreamError::InvalidString(vec![actual, expected], self.create_err(self.byte_pos())));
		} else {
			self.advance(1);
			Ok(())
		}
	}

	pub fn consume_string(&mut self, source: &[u8]) -> StreamResult<()> {
		if self.at_end() {
			return Err(StreamError::UnexpectedEndOfStream);
		}

		if !self.starts_with(source) {
			let len = source.len().min(self.source.len() - self.position);
			let actual = self.source[self.position..].chars().take(len).collect();
			let expected = std::str::from_utf8(source).unwrap().to_owned();

			return Err(StreamError::InvalidString(vec![actual, expected], self.create_err(self.byte_pos())));
		}

		self.advance(source.len());
		Ok(())
	}

	pub fn consume_str_chunk(&mut self) -> &'a str {
		if let Ok(b'&') = self.current_byte() {
			self.advance(1);
			if let Ok(name) = self.consume_xml_name() {
				if self.consume_byte(b';').is_ok() {
					match name.text {
						"quot" => return "\"",
						"amp" => return "&",
						"apos" => return "'",
						"lt" => return "<",
						"gt" => return ">",
						_ => {}
					};
				}
			}
		}
		self.consume_bytes(|_, b| b != b'&').text
	}

	pub fn consume_bytes<F>(&mut self, f: F) -> Span<'a>
	where
		F: Fn(&SvgStream, u8) -> bool,
	{
		let start = self.byte_pos();
		self.skip_bytes(f);
		self.slice_to_current(start)
	}

	pub fn skip_bytes<F>(&mut self, f: F)
	where
		F: Fn(&SvgStream, u8) -> bool,
	{
		while !self.at_end() {
			let c = self.current_byte_unchecked();
			if f(self, c) {
				self.advance(1);
			} else {
				break;
			}
		}
	}

	pub fn consume_ident(&'a mut self) -> Span<'a> {
		let start = self.position;
		self.skip_bytes(|_, c| c.is_ident());
		self.slice_to_current(start)
	}

	pub fn skip_xml_name(&mut self) -> StreamResult<()> {
		let start = self.byte_pos();
		let mut chars = self.chars();
		chars
			.next()
			.and_then(|c| {
				self.advance(c.len_utf8());
				c.is_xml_name_start().then_some(())
			})
			.ok_or(StreamError::InvalidXMLName(self.create_err(start)))?;

		chars.take_while(|c| c.is_xml_name()).for_each(|c| self.advance(c.len_utf8()));

		Ok(())
	}

	pub fn consume_xml_name(&mut self) -> StreamResult<Span<'a>> {
		let start = self.byte_pos();
		self.skip_xml_name()?;
		if start == self.byte_pos() {
			return Err(StreamError::InvalidXMLName(self.create_err(start)));
		}
		Ok(self.slice_to_current(start))
	}

	pub fn consume_qname(&mut self) -> StreamResult<QName<'a>> {
		let start = self.byte_pos();
		let mut chars = self.chars();
		chars
			.next()
			.and_then(|c| {
				self.advance(c.len_utf8());
				c.is_xml_name_start().then_some(())
			})
			.ok_or(StreamError::InvalidXMLName(self.create_err(start)))?;

		chars.by_ref().take_while(|c| c.is_xml_name() && *c != ':').for_each(|c| self.advance(c.len_utf8()));
		let begin = self.slice_to_current(start);

		if self.current_byte()? == b':' {
			self.advance(1);
			let local_start = self.byte_pos();
			chars
				.next()
				.and_then(|c| {
					self.advance(c.len_utf8());
					c.is_xml_name_start().then_some(())
				})
				.ok_or(StreamError::InvalidXMLName(self.create_err(local_start)))?;

			chars.take_while(|c| c.is_xml_name() && *c != ':').for_each(|c| self.advance(c.len_utf8()));
			Ok(QName {
				prefix: begin,
				local: self.slice_to_current(local_start),
				span: self.slice_to_current(start),
			})
		} else {
			Ok(QName {
				prefix: Span::from(""),
				local: begin,
				span: begin,
			})
		}
	}

	pub fn consume_equals(&mut self) -> StreamResult<()> {
		self.skip_spaces();
		self.consume_byte(b'=')?;
		self.skip_spaces();
		Ok(())
	}

	pub fn consume_quote(&mut self) -> StreamResult<u8> {
		let current = self.current_byte()?;
		if current == b'"' || current == b'\'' {
			self.advance(1);
			Ok(current)
		} else {
			Err(StreamError::InvalidQuote(self.create_err(self.byte_pos())))
		}
	}

	pub fn slice_to_current(&self, position: usize) -> Span<'a> {
		Span::from_range(self.source, position, self.position)
	}

	pub fn slice_tail(&self) -> Span<'a> {
		Span::from_range(self.source, self.position, self.source.len())
	}

	/// Parses integer number from the stream
	pub fn parse_integer(&mut self) -> StreamResult<i32> {
		self.skip_spaces();

		if self.at_end() {
			return Err(StreamError::InvalidNumber(self.create_err(self.byte_pos())));
		}

		let start = self.byte_pos();

		// Consume sign
		if self.current_byte()?.is_sign() {
			self.advance(1);
		}

		// The current char must be a digit
		if !self.current_byte()?.is_digit() {
			return Err(StreamError::InvalidNumber(self.create_err(start)));
		}

		self.skip_digits();

		// Use the default i32 parser now
		let s = self.slice_to_current(start);
		match i32::from_str(&s) {
			Ok(n) => Ok(n),
			Err(_) => Err(StreamError::InvalidNumber(self.create_err(start))),
		}
	}

	/// Parses integer from a list of numbers
	pub fn parse_list_integer(&mut self) -> StreamResult<i32> {
		if self.at_end() {
			return Err(StreamError::UnexpectedEndOfStream);
		}

		let n = self.parse_integer()?;
		self.skip_spaces();
		self.parse_list_separator();
		Ok(n)
	}

	/// Parses number or percent from the stream
	///
	/// Percent value will be converted to decimal
	pub fn parse_number_or_percent(&mut self) -> StreamResult<f64> {
		self.skip_spaces();

		let n = self.parse_number()?;
		if self.starts_with(b"%") {
			self.advance(1);
			Ok(n / 100.0)
		} else {
			Ok(n)
		}
	}

	/// Parses number or percent from a list of numbers and/or percents
	pub fn parse_list_number_or_percent(&mut self) -> StreamResult<f64> {
		if self.at_end() {
			return Err(StreamError::UnexpectedEndOfStream);
		}

		let l = self.parse_number_or_percent()?;
		self.skip_spaces();
		self.parse_list_separator();
		Ok(l)
	}

	/// Skips digits
	pub fn skip_digits(&mut self) {
		self.skip_bytes(|_, c| c.is_digit());
	}

	pub fn parse_list_separator(&mut self) {
		if self.match_current(b',') {
			self.advance(1);
		}
	}
}
