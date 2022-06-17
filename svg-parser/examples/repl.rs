use std::io::Write;
use svg_parser::parse;
fn main() {
	loop {
		let mut current = std::env::current_dir().unwrap();
		print!("Enter file path from \"{}\": ", current.to_str().unwrap());
		std::io::stdout().flush().unwrap();
		let mut input = String::new();
		std::io::stdin().read_line(&mut input).unwrap();
		input.pop();

		if input.is_empty() {
			input = String::from("svgs/tiger.svg");
		}
		current.push(input);

		let file = match std::fs::read_to_string(&current) {
			Ok(file) => file,
			Err(e) => {
				println!("Error: Invalid path {e} {current:?}\n");
				continue;
			}
		};
		println!("File {file}");
		parse(&file);
	}
}
