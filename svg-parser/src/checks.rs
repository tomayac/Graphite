/// Extension methods for XML-subset only operations.
pub trait XmlCharExt {
	fn is_xml_name_start(&self) -> bool;
	fn is_xml_name(&self) -> bool;
	fn is_xml_char(&self) -> bool;
}

impl XmlCharExt for char {
	fn is_xml_name_start(&self) -> bool {
		// ASCII
		if *self as u32 <= 128 {
			return match *self as u8 {
				b'A'..=b'Z' | b'a'..=b'z' | b':' | b'_' => true,
				_ => false,
			};
		}

		matches!(*self as u32, 0x0000C0..=0x0000D6
			| 0x0000D8..=0x0000F6
			| 0x0000F8..=0x0002FF
			| 0x000370..=0x00037D
			| 0x00037F..=0x001FFF
			| 0x00200C..=0x00200D
			| 0x002070..=0x00218F
			| 0x002C00..=0x002FEF
			| 0x003001..=0x00D7FF
			| 0x00F900..=0x00FDCF
			| 0x00FDF0..=0x00FFFD
			| 0x010000..=0x0EFFFF)
	}

	fn is_xml_name(&self) -> bool {
		// ASCII
		if *self as u32 <= 128 {
			return (*self as u8).is_xml_name();
		}

		match *self as u32 {
			0x0000B7
			| 0x0000C0..=0x0000D6
			| 0x0000D8..=0x0000F6
			| 0x0000F8..=0x0002FF
			| 0x000300..=0x00036F
			| 0x000370..=0x00037D
			| 0x00037F..=0x001FFF
			| 0x00200C..=0x00200D
			| 0x00203F..=0x002040
			| 0x002070..=0x00218F
			| 0x002C00..=0x002FEF
			| 0x003001..=0x00D7FF
			| 0x00F900..=0x00FDCF
			| 0x00FDF0..=0x00FFFD
			| 0x010000..=0x0EFFFF => true,
			_ => false,
		}
	}

	fn is_xml_char(&self) -> bool {
		match *self as u32 {
			0x000009 | 0x00000A | 0x00000D | 0x000020..=0x00D7FF | 0x00E000..=0x00FFFD | 0x010000..=0x10FFFF => true,
			_ => false,
		}
	}
}

/// Checks for a u8 byte representing a character
pub trait ByteExt {
	fn is_sign(&self) -> bool;
	fn is_digit(&self) -> bool;
	fn is_hex_digit(&self) -> bool;
	fn is_whitespace(&self) -> bool;
	fn is_letter(&self) -> bool;
	fn is_ident(&self) -> bool;
	fn is_xml_name(&self) -> bool;
}

impl ByteExt for u8 {
	fn is_sign(&self) -> bool {
		matches!(*self, b'+' | b'-')
	}

	fn is_digit(&self) -> bool {
		matches!(*self, b'0'..=b'9')
	}

	fn is_hex_digit(&self) -> bool {
		matches!(*self, b'0'..=b'9' | b'A'..=b'F' | b'a'..=b'f')
	}

	fn is_whitespace(&self) -> bool {
		matches!(*self, b' ' | b'\t' | b'\n' | b'\r')
	}

	fn is_letter(&self) -> bool {
		matches!(*self, b'A'..=b'Z' | b'a'..=b'z')
	}

	fn is_ident(&self) -> bool {
		matches!(*self, b'0'..=b'9' | b'A'..=b'Z' | b'a'..=b'z' | b'-' | b'_')
	}

	fn is_xml_name(&self) -> bool {
		matches!(*self, b'A'..=b'Z' | b'a'..=b'z'| b'0'..=b'9'| b':' | b'_' | b'-' | b'.')
	}
}
