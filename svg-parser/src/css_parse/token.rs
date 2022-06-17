use crate::{span::Span, number::Number};
/// https://drafts.csswg.org/css-syntax/#tokenization
#[derive(Debug)]
pub enum Token<'a> {Ident,
Function{
	name: Span<'a>,
},At{
	ident: Span<'a>,
},Hash {
	value: Span<'a>,
},
	/// "hello"
	String{
		value: Span<'a>,
	},Number{
		value: Number,
	},Percentage{
		value: Number,
	},Dimension{
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
pub struct CssToken{
	token_type: 
}

// https://drafts.csswg.org/css-syntax/#tokenizer-algorithms