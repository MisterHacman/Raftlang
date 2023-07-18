use crate::error::SyntaxError;
use crate::position::{Position, get_pos};
use crate::utils::both;

#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
	Identifier,
	Keyword,
	Parenthesis
}

#[derive(Clone)]
pub struct Token {
	pub class: TokenType,
	pub data: String,
	pub pos: Position
}

impl std::fmt::Debug for Token {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result { write!(f, "{:?}: {}", self.class, self.data) }
}

const NON_IDENTIFIERS: [u8; 5] = [b' ', b'\t', b'\n', b'(', b')'];
const KEYWORDS: [&str; 1] = ["fn"];

pub fn tokenize(buf: &[u8]) -> Result<Vec<Token>, SyntaxError> {
	let mut tokens: Vec<Token> = vec![];
	let mut i = 0;
	while i < buf.len() {
		match buf[i] {
			b' ' | b'\t' | b'\n' => {},
			b'(' | b')' => tokens.push(Token {
				class: TokenType::Parenthesis,
				data: (buf[i] as char).to_string(),
				pos: get_pos(buf, i, i + 1)
			}),
			b'\\' => tokens.push(Token { class: TokenType::Keyword, data: "\\".to_owned(), pos: get_pos(&buf, i, i + 1) }),
			_ => tokens.push(create_id(buf, &mut i))
		};
		i += 1;
	}
	Ok(tokens)
}

fn create_id(buf: &[u8], i: &mut usize) -> Token {
	let start = *i;
	let mut id = "".to_owned();
	while both(|| *i < buf.len(), || !NON_IDENTIFIERS.contains(&buf[*i])) {
		id.push(buf[*i] as char);
		*i += 1
	}
	*i -= 1;
	if KEYWORDS.contains(&id.as_str()) {
		Token { class: TokenType::Keyword, data: id, pos: get_pos(buf, start, *i) }
	} else {
		Token { class: TokenType::Identifier, data: id, pos: get_pos(buf, start, *i), }
	}
}
