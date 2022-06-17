use crate::{error::*, number::Number, span::Span, stream::SvgStream};
/// https://drafts.csswg.org/css-syntax/#tokenization
#[derive(Debug)]
pub enum TokenType<'a> {
	Ident,
	Function {
		name: Span<'a>,
	},
	At {
		ident: Span<'a>,
	},
	Hash {
		value: Span<'a>,
	},
	/// "hello"
	String {
		value: Span<'a>,
	},
	Number {
		value: Number,
	},
	Percentage {
		value: Number,
	},
	Dimension {
		value: Number,
	},
	Whitespace,
	Colon,
	Semicolon,
	Comma,
	OpenSquare,
	CloseSquare,
	OpenParen,
	CloseParen,
	OpenBrace,
	CloseBrace,
	Newline,
}
#[derive(Debug)]
pub struct CssToken<'a> {
	token_type: TokenType<'a>,
	span: Span<'a>,
}

// https://drafts.csswg.org/css-syntax/#tokenizer-algorithms
pub struct CssTokeniser<'a> {
	stream: SvgStream<'a>,
}

impl<'a> From<SvgStream<'a>> for CssTokeniser<'a> {
	fn from(stream: SvgStream<'a>) -> Self {
		Self { stream }
	}
}
impl<'a> Iterator for CssTokeniser<'a> {
	type Item = SvgResult<CssToken<'a>>;

	fn next(&mut self) -> Option<Self::Item> {
		self.parse_next()
	}
}

impl<'a> CssTokeniser<'a> {
	pub fn parse_next(&mut self) -> Option<SvgResult<CssToken<'a>>> {
		if self.stream.at_end() {
			return None;
		}
		let start = self.stream.byte_pos();
		match self.stream.current_byte_unchecked() {
			b'(' => Some(Self::parse_char_token(&mut self.stream, TokenType::OpenParen)),
			b')' => Some(Self::parse_char_token(&mut self.stream, TokenType::CloseParen)),
			b'[' => Some(Self::parse_char_token(&mut self.stream, TokenType::OpenSquare)),
			b']' => Some(Self::parse_char_token(&mut self.stream, TokenType::CloseSquare)),
			b'{' => Some(Self::parse_char_token(&mut self.stream, TokenType::OpenBrace)),
			b'}' => Some(Self::parse_char_token(&mut self.stream, TokenType::CloseBrace)),
			b':' => Some(Self::parse_char_token(&mut self.stream, TokenType::Colon)),
			b';' => Some(Self::parse_char_token(&mut self.stream, TokenType::Semicolon)),
			b',' => Some(Self::parse_char_token(&mut self.stream, TokenType::Comma)),
			b'\n' => Some(Self::parse_char_token(&mut self.stream, TokenType::Newline)),
			b'\r' => Some(Self::parse_return(&mut self.stream)),
			b'\'' | b'"' => Some(Self::parse_string(&mut self.stream).map_err(|e| SvgError(SvgErrorType::CssInvalidString, e, self.stream.create_err(start)))),
			_ => Some(Err(SvgError(SvgErrorType::UnknownToken, StreamError::default(), self.stream.create_err(start)))),
		}
	}
	fn parse_string(stream: &mut SvgStream<'a>) -> StreamResult<CssToken<'a>> {
		let start = stream.byte_pos();
		let quote = stream.consume_quote()?;
		let value = stream.consume_bytes(|_, c| c != quote);
		stream.consume_byte(quote)?;

		let span = stream.slice_to_current(start);
		Ok(CssToken {
			token_type: TokenType::String { value },
			span,
		})
	}
	fn parse_char_token(stream: &mut SvgStream<'a>, token_type: TokenType<'a>) -> SvgResult<CssToken<'a>> {
		let start = stream.byte_pos();
		stream.advance(1);
		let span = stream.slice_to_current(start);
		Ok(CssToken { token_type, span })
	}
	fn parse_return(stream: &mut SvgStream<'a>) -> SvgResult<CssToken<'a>> {
		let start = stream.byte_pos();
		stream.advance(1);
		// Sometimes \r is followed by \n for some reason (so new line is \r\n)
		if stream.match_current(b'\n') {
			stream.advance(1);
		}
		let span = stream.slice_to_current(start);
		Ok(CssToken { token_type: TokenType::Newline, span })
	}
}

#[test]
fn css_token() {
	let tokeniser: CssTokeniser = CssTokeniser::from(SvgStream::from(r#""hello"()"#));
	for token in tokeniser {
		println!("{token:?}");
	}
}
