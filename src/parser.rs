use crate::lexer::Token;
use crate::expr::{Expr, Sign, Type, parse_expr};

#[derive(Debug, Clone)]
pub enum Node {
	Print(Expr),
	Exit(Expr),
	Let {
		ident: String,
		expr: Expr,
		var_type: Type
	},
	Assign {
		ident: String,
		expr: Expr,
	},
	Scope(Vec<Node>)
}

pub struct ParserContext {
	pub tokens: Vec<Token>,
	pub i: usize,
}

impl ParserContext {
	#[inline]
	pub fn peek(self: &mut Self, ahead: usize) -> Token {
		self.tokens[self.i + ahead].clone()
	}
	#[inline]
	pub fn next_token(self: &mut Self) -> Token {
		let token = self.tokens[self.i].clone();
		self.i += 1;
		token
	}
	pub fn check_token(self: &mut Self, token: Token) -> bool {
		if self.peek(0) == token {
			return true;
		}
		return false;
	}
	pub fn expect_token(self: &mut Self, token: Token) -> Result<Token, String> {
		if self.check_token(token.clone()) {
			return Ok(self.next_token())
		} else {
			return Err(format!("Expected {:?}, but got {:?}", token, self.peek(0)));
		}
	}
}


pub fn parse(ctx: &mut ParserContext, scope: usize) -> Result<Vec<Node>, String> {
	let mut nodes = Vec::<Node>::new();
	
	while ctx.i < ctx.tokens.len() {
		let node = match ctx.next_token() {
			Token::Print => {
				ctx.expect_token(Token::OpenParen)?;
				let expr = parse_expr(ctx)?;
				ctx.expect_token(Token::CloseParen)?;
				Ok(Node::Print(expr))
			},
			Token::Exit => {
				ctx.expect_token(Token::OpenParen)?;
				let expr = parse_expr(ctx)?;
				ctx.expect_token(Token::CloseParen)?;
				Ok(Node::Exit(expr))
			},
			
			Token::OpenBrace => {
				Ok(Node::Scope(parse(ctx, scope + 1)?))
			},
			
			Token::CloseBrace => {
				if scope == 0 {
					return Err("Unexpected }, You may have a missing {.".to_string());
				}
				return Ok(nodes);
			},
			
			Token::Ident(ident) => {
				match ctx.next_token() {
					Token::Colon => {
						match ctx.check_token(Token::Equals) {
							true => {
								ctx.next_token();
								let expr = parse_expr(ctx)?;
								Ok(Node::Let {ident, expr, var_type: Type::Auto })
							},
							false => {
								match Type::from_token(ctx.next_token()) {
									Ok(var_type) => {
										ctx.expect_token(Token::Equals)?;
										let expr = parse_expr(ctx)?;
										Ok(Node::Let {ident, expr, var_type})
									}
									Err(e) => Err(e)
								}
							}
						}
					},
					Token::Equals => {
						Ok(Node::Assign { ident, expr: parse_expr(ctx)? })
					},
					Token::Plus|Token::Dash|Token::Star|Token::Slash => {
						ctx.i -= 1;
						let sign = Sign::from_token(&ctx.next_token())?;
						ctx.expect_token(Token::Equals)?;
						Ok(Node::Assign { 
							ident: ident.clone(),
							expr: Expr::SumExpr {
								sign,
								summands: vec!(
									Expr::Ident(ident),
									parse_expr(ctx)?
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
