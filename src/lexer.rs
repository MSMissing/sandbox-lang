#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token {
	None,
	Print,
	Ident(String),
	OpenParen,
	CloseParen,
	StringLit(String),
}

fn lex_keyword(i: &mut usize, code_bytes: &[u8]) -> Token {
	let mut keyword = Vec::<u8>::new();
	while (code_bytes[*i] as char).is_alphanumeric() {
		keyword.push(code_bytes[*i]);
		*i += 1;
	}
	return match String::from_utf8(keyword.clone()).unwrap().as_str() {
		"print" => Token::Print,
		_ => Token::Ident(String::from_utf8(keyword).unwrap())
	};
}

fn lex_string(i: &mut usize, code_bytes: &[u8]) -> String {
	let mut s = Vec::<u8>::new();
	*i += 1;
	while code_bytes[*i] != b'\"' {
		s.push(code_bytes[*i]);
		*i += 1;
	}
	return String::from_utf8(s).unwrap();
}

pub fn lex(code: String) -> Vec<Token> {
	let mut tokens = Vec::<Token>::new();
	let code_bytes = code.as_bytes();
	
	let mut i: usize = 0;
	while i < code_bytes.len() {
		if (code_bytes[i] as char).is_alphabetic() {
			tokens.push(lex_keyword(&mut i, code_bytes));
		} else {
			tokens.push(match code_bytes[i] {
				b'(' => Token::OpenParen,
				b')' => Token::CloseParen,
				b'\"' => Token::StringLit(lex_string(&mut i, code_bytes)),
				b'\n'|b' '|b'\t' => Token::None,
				_ => {panic!("Invalid token {}", code_bytes[i] as char);}
			});
			i += 1;
		}
	}
	
	i = 0;
	while i < tokens.len() {
		if tokens[i] == Token::None {
			tokens.remove(i);
		} else {
			i += 1;
		}
	}
	
	tokens
}
