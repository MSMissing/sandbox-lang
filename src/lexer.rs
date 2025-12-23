#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token {
	None,
	Print,
	Exit,
	Ident(String),
	TypeString,
	TypeBool,
	TypeInt,
	TypeAny,
	True,
	False,
	OpenParen,
	CloseParen,
	OpenBrace,
	CloseBrace,
	StringLit(String),
	IntLit(i64),
	Equals,
	Bang,
	Colon,
	Plus,
	Dash,
	Star,
	Slash,
	Semicolon,
	If,
	While,
}

fn lex_keyword(i: &mut usize, code_bytes: &[u8]) -> Token {
	let mut keyword = Vec::<u8>::new();
	while (code_bytes[*i] as char).is_alphanumeric() {
		keyword.push(code_bytes[*i]);
		*i += 1;
	}
	match keyword.as_slice() {
		b"print" => Token::Print,
		b"exit" => Token::Exit,
		b"if" => Token::If,
		b"while" => Token::While,
		b"String" => Token::TypeString,
		b"Int" => Token::TypeInt,
		b"Bool" => Token::TypeBool,
		b"Any" => Token::TypeAny,
		b"true" => Token::True,
		b"false" => Token::False,
		_ => Token::Ident(String::from_utf8(keyword).unwrap())
	}
}

fn lex_string(i: &mut usize, code_bytes: &[u8]) -> String {
	let mut s = Vec::<u8>::new();
	*i += 1;
	while code_bytes[*i] != b'\"' {
		s.push(code_bytes[*i]);
		*i += 1;
	}
	String::from_utf8(s).unwrap()
}

fn lex_int(i: &mut usize, code_bytes: &[u8]) -> i64 {
	let mut s = Vec::<u8>::new();
	while code_bytes[*i].is_ascii_digit() {
		s.push(code_bytes[*i]);
		*i += 1;
	}
	str::from_utf8(&s).unwrap().parse().unwrap()
}

pub fn lex(code: String) -> Result<Vec<Token>, String> {
	let mut tokens = Vec::<Token>::new();
	let code_bytes = code.as_bytes();
	
	let mut i: usize = 0;
	while i < code_bytes.len() {
		if code_bytes[i].is_ascii_alphabetic() {
			tokens.push(lex_keyword(&mut i, code_bytes));
		} else if code_bytes[i].is_ascii_digit() {
			tokens.push(Token::IntLit(lex_int(&mut i, code_bytes)));
		} else {
			tokens.push(match code_bytes[i] {
				b'(' => Token::OpenParen,
				b')' => Token::CloseParen,
				b'{' => Token::OpenBrace,
				b'}' => Token::CloseBrace,
				b'\"' => Token::StringLit(lex_string(&mut i, code_bytes)),
				b'\n'|b' '|b'\t' => Token::None,
				b':' => Token::Colon,
				b'=' => Token::Equals,
				b'+' => Token::Plus,
				b'-' => Token::Dash,
				b'*' => Token::Star,
				b'/' => Token::Slash,
				b';' => Token::Semicolon,
				b'!' => Token::Bang,
				_ => return Err(format!("Invalid token {}", code_bytes[i] as char))
			});
			i += 1;
		}
	}
	
	// remove all of Token::None
	i = 0;
	while i < tokens.len() {
		if tokens[i] == Token::None {
			tokens.remove(i);
		} else {
			i += 1;
		}
	}
	
	Ok(tokens)
}
