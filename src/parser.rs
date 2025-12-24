use crate::expr::{Expr, Sign, Type, parse_expr};
use crate::lexer::Token;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Node {
	Print(Expr),
	Exit(Expr),
	Let {
		ident: String,
		expr: Expr,
		var_type: Type,
	},
	Assign {
		ident: String,
		expr: Expr,
	},
	If {
		expr: Expr,
		body: Box<Node>,
	},
	While {
		expr: Expr,
		body: Box<Node>,
	},
	Scope(Vec<Node>),
}

pub struct ParserContext {
	pub tokens: Vec<Token>,
	pub i: usize,
}

impl ParserContext {
	pub fn peek(&self, ahead: usize) -> Result<Token, String> {
		if self.i + ahead >= self.tokens.len() {
			return Err(String::from("Index out of bounds"));
		}
		Ok(self.tokens[self.i + ahead].clone())
	}
	pub fn next_token(&mut self) -> Result<Token, String> {
		if self.i >= self.tokens.len() {
			return Err(String::from("Index out of bounds"));
		}
		let token = self.tokens[self.i].clone();
		self.i += 1;
		Ok(token)
	}
	pub fn check_token(&mut self, token: Token) -> bool {
		match self.peek(0) {
			Ok(t) => t == token,
			Err(_) => false,
		}
	}
	pub fn expect_token(&mut self, token: Token) -> Result<Token, String> {
		if self.check_token(token.clone()) {
			Ok(self.next_token()?)
		} else {
			Err(format!("Expected {:?}, but got {:?}", token, self.peek(0)))
		}
	}

	pub fn new(tokens: Vec<Token>) -> Self {
		Self { tokens, i: 0 }
	}
}

pub fn parse(ctx: &mut ParserContext, scope: usize) -> Result<Vec<Node>, String> {
	let mut nodes = Vec::<Node>::new();

	while ctx.i < ctx.tokens.len() {
		let node = match ctx.next_token()? {
			Token::Print => {
				ctx.expect_token(Token::OpenParen)?;
				let expr = parse_expr(ctx)?;
				ctx.expect_token(Token::CloseParen)?;
				Ok(Node::Print(expr))
			}
			Token::Exit => {
				ctx.expect_token(Token::OpenParen)?;
				let expr = parse_expr(ctx)?;
				ctx.expect_token(Token::CloseParen)?;
				Ok(Node::Exit(expr))
			}

			Token::OpenBrace => Ok(Node::Scope(parse(ctx, scope + 1)?)),

			Token::CloseBrace => {
				if scope == 0 {
					return Err("Unexpected }, You may have a missing {.".to_string());
				}
				return Ok(nodes);
			}

			Token::If => {
				let expr = parse_expr(ctx)?;
				ctx.expect_token(Token::OpenBrace)?;
				Ok(Node::If {
					expr,
					body: Box::new(Node::Scope(parse(ctx, scope + 1)?)),
				})
			}

			Token::While => {
				let expr = parse_expr(ctx)?;
				ctx.expect_token(Token::OpenBrace)?;
				Ok(Node::While {
					expr,
					body: Box::new(Node::Scope(parse(ctx, scope + 1)?)),
				})
			}

			Token::Ident(ident) => match ctx.next_token()? {
				Token::Colon => match ctx.check_token(Token::Equals) {
					true => {
						ctx.next_token()?;
						let expr = parse_expr(ctx)?;
						Ok(Node::Let {
							ident,
							expr,
							var_type: Type::Auto,
						})
					}
					false => match Type::from_token(ctx.next_token()?) {
						Ok(var_type) => {
							ctx.expect_token(Token::Equals)?;
							let expr = parse_expr(ctx)?;
							Ok(Node::Let {
								ident,
								expr,
								var_type,
							})
						}
						Err(e) => Err(e),
					},
				},
				Token::Equals => Ok(Node::Assign {
					ident,
					expr: parse_expr(ctx)?,
				}),
				Token::Plus | Token::Dash | Token::Star | Token::Slash => {
					ctx.i -= 1;
					let sign = Sign::from_token(&ctx.next_token()?)?;
					ctx.expect_token(Token::Equals)?;
					Ok(Node::Assign {
						ident: ident.clone(),
						expr: Expr::Sum {
							sign,
							summands: vec![Expr::Ident(ident), parse_expr(ctx)?],
						},
					})
				}
				_ => Err(format!("Unexpected identifier {}", ident)),
			},
			_ => Err(format!("Unexpected token {:?}", ctx.peek(0))),
		}?;
		nodes.push(node);
	}

	Ok(nodes)
}
