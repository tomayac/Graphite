use std::str::FromStr;

use crate::{checks::ByteExt, error::StreamError, span::ErrorPos, stream::SvgStream};

/// An [SVG number](https://www.w3.org/TR/SVG2/types.html#InterfaceSVGNumber).
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Number(pub f64);

impl std::str::FromStr for Number {
	type Err = StreamError;

	fn from_str(text: &str) -> Result<Self, Self::Err> {
		let mut s = SvgStream::from(text);
		let n = s.parse_number()?;
		s.skip_spaces();
		if !s.at_end() {
			return Err(StreamError::UnexpectedData(s.create_err(s.byte_pos())));
		}

		Ok(Self(n))
	}
}

impl<'a> SvgStream<'a> {
	/// Parses number from the stream.
	///
	/// This method will detect a number length and then
	/// will pass a substring to the `f64::from_str` method.
	pub fn parse_number(&mut self) -> Result<f64, StreamError> {
		self.skip_spaces();

		let start = self.byte_pos();

		if self.at_end() {
			return Err(StreamError::InvalidNumber(self.create_err(start)));
		}

		self.parse_number_impl().map_err(|_| StreamError::InvalidNumber(self.create_err(start)))
	}

	fn parse_number_impl(&mut self) -> Result<f64, StreamError> {
		let start = self.byte_pos();

		let mut c = self.current_byte()?;

		// Consume sign
		if c.is_sign() {
			self.advance(1);
			c = self.current_byte()?;
		}

		// Consume integer
		match c {
			b'0'..=b'9' => self.skip_digits(),
			b'.' => {}
			_ => return Err(StreamError::InvalidNumber(ErrorPos::default())),
		}

		// Consume decimal
		if let Ok(b'.') = self.current_byte() {
			self.advance(1);
			self.skip_digits();
		}

		if let Ok(c) = self.current_byte() {
			if matches!(c, b'e' | b'E') {
				if let Ok(c2) = self.peek() {
					// Check for `em`/`ex`.
					if c2 != b'm' && c2 != b'x' {
						self.advance(1);

						match self.current_byte()? {
							b'+' | b'-' => {
								self.advance(1);
								self.skip_digits();
							}
							b'0'..=b'9' => self.skip_digits(),
							_ => {
								return Err(StreamError::InvalidNumber(ErrorPos::default()));
							}
						}
					}
				}
			}
		}

		let s = self.slice_to_current(start);

		// Use the default f64 parser now.
		if let Ok(n) = f64::from_str(&s) {
			// inf, nan, etc. are an error.
			if n.is_finite() {
				return Ok(n);
			}
		}

		Err(StreamError::InvalidNumber(ErrorPos::default()))
	}

	/// Parses number from a list of numbers.
	pub fn parse_list_number(&mut self) -> Result<f64, StreamError> {
		if self.at_end() {
			return Err(StreamError::UnexpectedEndOfStream);
		}

		let n = self.parse_number()?;
		self.skip_spaces();
		self.parse_list_separator();
		Ok(n)
	}
}

/// [`<list-of-numbers>`]: https://www.w3.org/TR/SVG2/types.html#InterfaceSVGNumberList
#[derive(Clone, PartialEq, Debug)]
pub struct NumberListParser<'a>(SvgStream<'a>);

impl<'a> From<&'a str> for NumberListParser<'a> {
	#[inline]
	fn from(v: &'a str) -> Self {
		NumberListParser(SvgStream::from(v))
	}
}

impl<'a> Iterator for NumberListParser<'a> {
	type Item = Result<f64, StreamError>;

	fn next(&mut self) -> Option<Self::Item> {
		if self.0.at_end() {
			None
		} else {
			let v = self.0.parse_list_number();
			if v.is_err() {
				self.0.jump_to_end();
			}

			Some(v)
		}
	}
}

#[cfg(test)]
mod tests {
	use crate::SvgStream;

	macro_rules! test_parse {
		($name:ident, $text:expr, $result:expr) => {
			#[test]
			fn $name() {
				let mut s = SvgStream::from($text);
				assert_eq!(s.parse_number().unwrap(), $result);
			}
		};
	}

	test_parse!(parse_0, "0", 0.0);
	test_parse!(parse_1, "1", 1.0);
	test_parse!(parse_negitive, "-1", -1.0);
	test_parse!(parse_whitespace, " -1 ", -1.0);
	test_parse!(parse_double_whitespace, "  1  ", 1.0);
	test_parse!(parse_decimal, ".4", 0.4);
	test_parse!(parse_negitive_decimal, "-.4", -0.4);
	test_parse!(parse_text_1, "-.4text", -0.4);
	test_parse!(parse_text_2, "-.01 text", -0.01);
	test_parse!(parse_space_seperator, "-.01 4", -0.01);
	test_parse!(parse_small, ".0000000000008", 0.0000000000008);
	test_parse!(parse_large, "1000000000000", 1000000000000.0);
	test_parse!(parse_long_decimal, "123456.123456", 123456.123456);
	test_parse!(parse_explicit_pos, "+10", 10.0);
	test_parse!(parse_exp_1, "1e2", 100.0);
	test_parse!(parse_exp_2, "1e+2", 100.0);
	test_parse!(parse_e_capital, "1E2", 100.0);
	test_parse!(parse_negitive_exp, "1e-2", 0.01);
	test_parse!(parse_exp_text, "1ex", 1.0);
	test_parse!(parse_text_3, "1em", 1.0);
	test_parse!(parse_long, "12345678901234567890", 12345678901234567000.0);
	test_parse!(parse_trailing_point, "0.", 0.0);
	test_parse!(parse_exp_3, "1.3e-2", 0.013);
	test_parse!(parse_24, "1e", 1.0);

	macro_rules! test_p_err {
		($name:ident, $text:expr) => {
			#[test]
			fn $name() {
				let mut s = SvgStream::from($text);
				assert_eq!(s.parse_number().unwrap_err().to_string(), "invalid number at position 1");
			}
		};
	}

	test_p_err!(parse_err_1, "q");
	test_p_err!(parse_err_2, "");
	test_p_err!(parse_err_3, "-");
	test_p_err!(parse_err_4, "+");
	test_p_err!(parse_err_5, "-q");
	test_p_err!(parse_err_6, ".");
	test_p_err!(parse_err_7, "99999999e99999999");
	test_p_err!(parse_err_8, "-99999999e99999999");
}
