#![feature(result_option_inspect)]
#![feature(cow_is_borrowed)]
mod checks;
mod error;

mod number;
mod span;
mod stream;
mod xml_parse;
mod css_parse;

use stream::SvgStream;

pub fn parse(svg: &str) {
	xml_parse::parse_svg(svg).unwrap();
}

// #[cfg(test)]
// mod tests {

// 	use super::*;

// 	#[test]
// 	fn it_works() {
// 		parse("<hello>");
// 	}
// }
