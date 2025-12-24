#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum Token {
	#[default]
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

pub struct LexerContext {
	code_bytes: Vec<u8>,
	idx: usize,
}

impl LexerContext {
	pub fn new(code: String) -> Self {
		Self {
			code_bytes: code.into_bytes(),
			idx: 0,
		}
	}
	pub fn next(&mut self) -> Result<u8, String> {
		if self.idx >= self.code_bytes.len() {
			Err(String::from("Unexpected EOF"))
		} else {
			self.idx += 1;
			Ok(self.code_bytes[self.idx - 1])
		}
	}
	pub fn peek(&self, lookahead: usize) -> Result<u8, String> {
		if self.idx + lookahead >= self.code_bytes.len() {
			Err(String::from("Unexpected EOF"))
		} else {
			Ok(self.code_bytes[self.idx + lookahead])
		}
	}
}

fn lex_keyword(ctx: &mut LexerContext) -> Result<Token, String> {
	let mut keyword = Vec::<u8>::new();
	while match ctx.peek(0) {
		Ok(c) => c.is_ascii_alphabetic(),
		Err(_) => false
	} {
		keyword.push(ctx.next()?);
	}
	Ok(match keyword.as_slice() {
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
		_ => Token::Ident(String::from_utf8(keyword).unwrap()),
	})
}

fn lex_string(ctx: &mut LexerContext) -> Result<String, String> {
	let mut s = Vec::<u8>::new();
	ctx.next()?;
	while ctx.peek(0)? != b'\"' {
		s.push(ctx.next()?);
	}
	String::from_utf8(s).or(Err(String::from("Invalid utf-8")))
}

fn lex_int(ctx: &mut LexerContext) -> Result<i64, String> {
	let mut s = Vec::<u8>::new();
	while match ctx.peek(0) {
		Ok(c) => c.is_ascii_digit(),
		Err(_) => false
	} {
		s.push(ctx.peek(0)?);
		ctx.idx += 1;
	}
	let str_v = str::from_utf8(&s).or(Err(String::from("Invalid UTF-8")));
	println!("{:?}", str_v);
	let r = str_v?.parse();
	println!("{:?}", r);
	r.or(Err(format!(
		"Could not parse string {} into Int",
		String::from_utf8(s).or(Err(String::from("This should never happen.")))?
	)))
}

pub fn lex(ctx: &mut LexerContext) -> Result<Vec<Token>, String> {
	let mut tokens = Vec::<Token>::new();

	while ctx.idx < ctx.code_bytes.len() {
		let c = ctx.peek(0)?;
		if c.is_ascii_alphabetic() {
			tokens.push(lex_keyword(ctx)?);
		} else if c.is_ascii_digit() {
			tokens.push(Token::IntLit(lex_int(ctx)?));
		} else {
			tokens.push(match c {
				b'(' => Token::OpenParen,
				b')' => Token::CloseParen,
				b'{' => Token::OpenBrace,
				b'}' => Token::CloseBrace,
				b'\"' => Token::StringLit(lex_string(ctx)?),
				b'\n' | b' ' | b'\t' => Token::None,
				b':' => Token::Colon,
				b'=' => Token::Equals,
				b'+' => Token::Plus,
				b'-' => Token::Dash,
				b'*' => Token::Star,
				b'/' => Token::Slash,
				b';' => Token::Semicolon,
				b'!' => Token::Bang,
				_ => return Err(format!("Invalid token {}", ctx.peek(0)? as char)),
			});
			ctx.next()?;
		}
	}

	// remove all of Token::None
	let mut i = 0;
	while i < tokens.len() {
		if tokens[i] == Token::None {
			tokens.remove(i);
		} else {
			i += 1;
		}
	}

	Ok(tokens)
}
