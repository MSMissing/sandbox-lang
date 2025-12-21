use crate::lexer::Token;
use crate::parser::{ParserContext};

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

type Precedence = i8;

impl Sign {
	pub fn from_token(token: Token) -> Result<Self, String> {
		match token {
			Token::Plus      => Ok(Sign::Add),
			Token::Dash      => Ok(Sign::Subtract),
			Token::Star      => Ok(Sign::Multiply),
			Token::Slash     => Ok(Sign::Divide),
			Token::Semicolon => Ok(Sign::Concat),
			
			_ => Err(format!("Expected sign, but got {:?}", token))
		}
	}
	pub fn get_precedence(self: &Self) -> Precedence {
		match self {
			Sign::Add      => 0,
			Sign::Subtract => 0,
			Sign::Multiply => 1,
			Sign::Divide   => 1,
			Sign::Concat   => 0,
			_ => -1
		}
	}
	pub fn is_sign(token: Token) -> bool {
		match token {
			Token::Plus|Token::Dash|Token::Star|Token::Slash|Token::Semicolon => true,
			_ => false
		}
	}
}

pub fn parse_expr(ctx: &mut ParserContext) -> Result<Expr, String> {
	return parse_expr_1(ctx. parse_primary(ctx)?, 0)?;
}

pub fn parse_expr_1(ctx: &mut ParserContext, lhs: Expr, min_precedence: Precedence) -> Result<Expr, String> {
	let lookahead = ctx.peek(0);
	
	while match Sign::is_sign(lookahead) {
		true => Sign::from_token(lookahead).get_precedence() >= min_precedence,
		false => false
	} {
		
	}
}


pub fn parse_primary(ctx: &mut ParserContext) -> Result<Expr, String> {
	let expr = match ctx.next_token() {
		Token::StringLit(strlit) => Ok(Expr::StringLit(strlit)),
		Token::IntLit(intlit) => Ok(Expr::Int(intlit)),
		Token::Ident(ident) => Ok(Expr::Ident(ident)),
		_ => Err(format!("Expected expression, got {:?}", ctx.peek(0)))
	}?;
	Ok(Expr)
}
