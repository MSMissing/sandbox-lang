use crate::lexer::Token;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expr {
	SumExpr {
		sign: Sign,
		summands: Vec<Expr>
	},
	StringLit(String),
	Int(i64),
	Ident(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Sign {
	Add,
	Subtract,
	Multiply,
	Divide,
	Concat,
}

impl Sign {
	fn from_token(token: Token) -> Result<Self, String> {
		match token {
			Token::Plus      => Ok(Sign::Add),
			Token::Dash      => Ok(Sign::Subtract),
			Token::Star      => Ok(Sign::Multiply),
			Token::Slash     => Ok(Sign::Divide),
			Token::Semicolon => Ok(Sign::Concat),
			
			_ => Err(format!("Expected sign, but got {:?}", token))
		}
	}
}

#[derive(Debug, Clone)]
pub enum Node {
	Print(Expr),
	Exit(Expr),
	Let {
		ident: String,
		expr: Expr
	},
	Assign {
		ident: String,
		expr: Expr
	}
}

struct ParserContext {
	tokens: Vec<Token>,
	i: usize,
}

impl ParserContext {
	#[inline]
	fn peek(self: &mut Self, ahead: usize) -> Token {
		self.tokens[self.i + ahead].clone()
	}
	#[inline]
	fn next_token(self: &mut Self) -> Token {
		let token = self.tokens[self.i].clone();
		self.i += 1;
		token
	}
}

fn expect_token(ctx: &mut ParserContext, token: Token) -> Result<Token, String> {
	if check_token(ctx, token.clone()) {
		return Ok(ctx.next_token())
	} else {
		return Err(format!("Expected {:?}, but got {:?}", token, ctx.peek(0)));
	}
}

fn check_token(ctx: &ParserContext, token: Token) -> bool {
	if ctx.tokens[ctx.i].clone() == token {
		return true;
	}
	return false;
}

fn parse_expr(ctx: &mut ParserContext) -> Result<Expr, String> {
	let expr = match ctx.next_token() {
		Token::StringLit(strlit) => Ok(Expr::StringLit(strlit)),
		Token::IntLit(intlit) => Ok(Expr::Int(intlit)),
		Token::Ident(ident) => Ok(Expr::Ident(ident)),
		_ => Err(format!("Expected expression, got {:?}", ctx.peek(0)))
	}?;
	
	if ctx.i >= ctx.tokens.len() {
		return Ok(expr);
	}
	match ctx.peek(0) {
		Token::Plus|Token::Dash|Token::Star|Token::Slash|Token::Semicolon => {
			let sign = Sign::from_token(ctx.next_token())?;
			let sum = Expr::SumExpr {
				sign: sign,
				summands: vec!(expr, parse_expr(ctx)?)
			};
			
			Ok(sum)
		},
		_ => Ok(expr)
	}
}

pub fn parse(tokens: Vec<Token>) -> Result<Vec<Node>, String> {
	let mut nodes = Vec::<Node>::new();
	let mut ctx = ParserContext { tokens, i: 0 };
	
	while ctx.i < ctx.tokens.len() {
		let node = match ctx.next_token() {
			Token::Print => {
				expect_token(&mut ctx, Token::OpenParen)?;
				let expr = parse_expr(&mut ctx)?;
				expect_token(&mut ctx, Token::CloseParen)?;
				Ok(Node::Print(expr))
			},
			Token::Exit => {
				expect_token(&mut ctx, Token::OpenParen)?;
				let expr = parse_expr(&mut ctx)?;
				expect_token(&mut ctx, Token::CloseParen)?;
				Ok(Node::Exit(expr))
			},
			Token::Ident(ident) => {
				match ctx.next_token() {
					Token::Colon => {
						expect_token(&mut ctx, Token::Equals)?;
						let expr = parse_expr(&mut ctx)?;
						Ok(Node::Let {ident: ident, expr: expr})
					},
					Token::Equals => {
						Ok(Node::Assign { ident, expr: parse_expr(&mut ctx)? })
					},
					Token::Plus|Token::Dash|Token::Star|Token::Slash => {
						ctx.i -= 1;
						let sign = Sign::from_token(ctx.next_token())?;
						expect_token(&mut ctx, Token::Equals)?;
						Ok(Node::Assign { 
							ident: ident.clone(),
							expr: Expr::SumExpr {
								sign,
								summands: vec!(
									Expr::Ident(ident),
									parse_expr(&mut ctx)?
								)
							}
						})
					},
					
					_ => Err(format!("Unexpected identifier {}", ident))
				}
			}
			_ => Err(format!("Unexpected token {:?}", ctx.peek(0)))
		}?;
		nodes.push(node);
	}
	
	return Ok(nodes);
}
