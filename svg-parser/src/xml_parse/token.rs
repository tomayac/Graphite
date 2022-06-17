use crate::{
	error::{StreamError, StreamResult, SvgError, SvgErrorType, SvgResult},
	span::{QName, ResolvedQName, Span},
	stream::SvgStream,
};
#[derive(Debug)]
pub enum ElementControl<'a> {
	/// <bob>
	Open,
	/// </bob>
	Close(QName<'a>),
	// <bob />
	EmptyTag,
}
#[derive(Debug)]
pub struct Attribute<'a> {
	pub qname: QName<'a>,
	pub value: Span<'a>,
	pub span: Span<'a>,
}
#[derive(Debug)]
pub struct ProcessingInstruction<'a> {
	target: Span<'a>,
	content: Option<Span<'a>>,
}
#[derive(Debug)]
pub enum Token<'a> {
	/// <?xml version="1.0" encoding="UTF-8" standalone="no"?>
	ProcessingInstruction {
		processing_instruction: ProcessingInstruction<'a>,
		span: Span<'a>,
	},
	ElementStart {
		qname: QName<'a>,
		span: Span<'a>,
	},
	/// world="hello"
	Attribute(Attribute<'a>),
	/// Part of tag for element
	ElementControl {
		control: ElementControl<'a>,
		span: Span<'a>,
	},
	/// global text
	Text {
		span: Span<'a>,
	},
}

#[derive(PartialEq, Eq)]
enum State {
	Elements,
	Attributes,
	AfterElements,
	End,
}

pub struct Tokeniser<'a> {
	stream: SvgStream<'a>,
	state: State,
	depth: usize,
}

impl<'a> From<SvgStream<'a>> for Tokeniser<'a> {
	fn from(stream: SvgStream<'a>) -> Self {
		Tokeniser {
			stream,
			state: State::Elements,
			depth: 0,
		}
	}
}
impl<'a> Iterator for Tokeniser<'a> {
	type Item = SvgResult<Token<'a>>;

	fn next(&mut self) -> Option<SvgResult<Token<'a>>> {
		self.parse_next()
	}
}
impl<'a> Tokeniser<'a> {
	pub fn parse_next(&mut self) -> Option<SvgResult<Token<'a>>> {
		if self.stream.at_end() {
			return None;
		}
		let start = self.stream.byte_pos();
		match self.state {
			State::Elements => match self.stream.current_byte() {
				Ok(b'<') => match self.stream.peek() {
					Ok(b'/') => {
						if self.depth == 0 {
							//return Some(Err(SvgParseError::InvalidElement));
						} else {
							self.depth -= 1;
						}

						if self.depth == 0 {
							self.state = State::AfterElements;
						}
						Some(Self::parse_close_element(&mut self.stream).map_err(|e| SvgError(SvgErrorType::InvalidElement, e, self.stream.create_err(start))))
					}
					Ok(b'?') => Some(Self::parse_processor(&mut self.stream).map_err(|e| SvgError(SvgErrorType::InvalidProcessor, e, self.stream.create_err(start)))),
					Ok(b'!') => {
						if self.stream.starts_with(b"<!--") {
							self.stream.skip_spaces();
							return self.parse_next();
						} else {
							Some(Err(SvgError(SvgErrorType::UnknownToken, StreamError::default(), self.stream.create_err(start))))
						}
					}
					Ok(_) => {
						self.state = State::Attributes;
						Some(Self::parse_start_element(&mut self.stream).map_err(|e| SvgError(SvgErrorType::InvalidElement, e, self.stream.create_err(start))))
					}
					Err(_) => None,
				},
				Ok(_) => Some(Self::consume_text(&mut self.stream)),
				Err(_) => None,
			},
			State::Attributes => {
				let attribute = Self::parse_attribute(&mut self.stream);
				if let Ok(Token::ElementControl { control, .. }) = &attribute {
					if let ElementControl::Open = control {
						self.depth += 1;
					}
					if self.depth == 0 {
						self.state = State::AfterElements
					} else {
						self.state = State::Elements;
					}
				}

				Some(attribute.map_err(|e| SvgError(SvgErrorType::InvalidAttribute, e, self.stream.create_err(start))))
			}
			State::AfterElements => {
				self.stream.skip_spaces();
				if start != self.stream.byte_pos() {
					self.parse_next()
				} else if self.stream.starts_with(b"<?") {
					Some(Self::parse_processor(&mut self.stream).map_err(|e| SvgError(SvgErrorType::InvalidProcessor, e, self.stream.create_err(start))))
				} else {
					Some(Err(SvgError(SvgErrorType::UnknownToken, StreamError::default(), self.stream.create_err(start))))
				}
			}
			State::End => None,
		}
	}
	fn parse_start_element(stream: &mut SvgStream<'a>) -> StreamResult<Token<'a>> {
		let start = stream.byte_pos();
		stream.advance(1);
		let qname = stream.consume_qname()?;
		let span = stream.slice_to_current(start);
		Ok(Token::ElementStart { qname, span })
	}
	fn parse_attribute(stream: &mut SvgStream<'a>) -> StreamResult<Token<'a>> {
		stream.skip_spaces();
		let start = stream.byte_pos();

		match stream.current_byte()? {
			b'/' => {
				stream.advance(1);
				stream.consume_byte(b'>')?;
				let span = stream.slice_to_current(start);
				return Ok(Token::ElementControl {
					control: ElementControl::EmptyTag,
					span,
				});
			}
			b'>' => {
				stream.advance(1);
				let span = stream.slice_to_current(start);
				return Ok(Token::ElementControl { control: ElementControl::Open, span });
			}
			_ => {}
		}

		let qname = stream.consume_qname()?;
		stream.consume_equals()?;
		let quote = stream.consume_quote()?;
		let value = stream.consume_bytes(|_, c| c != quote);
		stream.consume_byte(quote)?;
		let span = stream.slice_to_current(start);
		Ok(Token::Attribute(Attribute { qname, value, span }))
	}
	fn consume_text(stream: &mut SvgStream<'a>) -> SvgResult<Token<'a>> {
		let span = stream.consume_bytes(|_, c| c != b'<');

		Ok(Token::Text { span })
	}

	fn parse_close_element(stream: &mut SvgStream<'a>) -> StreamResult<Token<'a>> {
		let start = stream.byte_pos();
		stream.advance(2);
		let qname = stream.consume_qname()?;
		stream.skip_spaces();
		stream.consume_byte(b'>')?;
		let span = stream.slice_to_current(start);
		Ok(Token::ElementControl {
			control: ElementControl::Close(qname),
			span,
		})
	}
	fn parse_processor(stream: &mut SvgStream<'a>) -> StreamResult<Token<'a>> {
		let start = stream.byte_pos();
		stream.advance(2);
		let target = stream.consume_xml_name()?;
		stream.skip_spaces();
		let content = stream.consume_bytes(|stream, _| !stream.match_current(b'?') || !stream.peek().map(|c| c == b'>').unwrap_or_default());
		let content = (!content.is_empty()).then_some(content);
		stream.consume_string(b"?>")?;
		let span = stream.slice_to_current(start);
		let processing_instruction = ProcessingInstruction { target, content };
		Ok(Token::ProcessingInstruction { processing_instruction, span })
	}
}

#[test]

fn token() {
	let tokeniser: Tokeniser = Tokeniser::from(SvgStream::from(
		r#"
<?bob jeff2?><content><ns:d bob='hello  worldy'></ns:d><football flyer='2' higher='4' /><p> Hello world </p></content>"#,
	));
	for token in tokeniser {
		println!("{token:?}");
	}
}
