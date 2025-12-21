use crate::lexer::Token;
use crate::parser::ParserContext;

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
	pub fn from_token(token: &Token) -> Result<Self, String> {
		match *token {
			Token::Plus      => Ok(Sign::Add),
			Token::Dash      => Ok(Sign::Subtract),
			Token::Star      => Ok(Sign::Multiply),
			Token::Slash     => Ok(Sign::Divide),
			Token::Semicolon => Ok(Sign::Concat),
			
			_                => Err(format!("Expected sign, but got {:?}", token))
		}
	}
	pub fn get_precedence(self: &Self) -> Precedence {
		match self {
			Sign::Add      => 1,
			Sign::Subtract => 1,
			Sign::Multiply => 2,
			Sign::Divide   => 2,
			Sign::Concat   => 1,
		}
	}
	pub fn is_sign(token: &Token) -> bool {
		match token {
			Token::Plus|Token::Dash|Token::Star|Token::Slash|Token::Semicolon => true,
			_ => false
		}
	}
}

pub fn parse_expr(ctx: &mut ParserContext) -> Result<Expr, String> {
	let primary = parse_primary(ctx)?;
	return parse_expr_1(ctx, primary, 0);
}

pub fn parse_expr_1(ctx: &mut ParserContext, lhs: Expr, min_precedence: Precedence) -> Result<Expr, String> {
	let mut lookahead = ctx.peek(0);
	if !Sign::is_sign(&lookahead) {
		return Ok(lhs);
	};
	let mut expr = lhs.clone();
	while match Sign::is_sign(&lookahead) {
		true => Sign::from_token(&lookahead).unwrap().get_precedence() >= min_precedence,
		false => false
	} {
		let op = Sign::from_token(&lookahead)?;
		ctx.next_token();
		let mut rhs = parse_primary(ctx)?;
		
		lookahead = ctx.peek(0);
		while match Sign::is_sign(&lookahead) {
			true => Sign::from_token(&lookahead).unwrap().get_precedence() > op.get_precedence(),
			false => false
		} {
			rhs = parse_expr_1(ctx, rhs, op.get_precedence() + 
						match Sign::from_token(&lookahead).unwrap().get_precedence() > op.get_precedence() {
							true => 1,
							false => 0
						})?;
			lookahead = ctx.peek(0);
		}
		expr = Expr::SumExpr { sign: op, summands: vec!(expr, rhs) };
	}
	Ok(expr)
}


pub fn parse_primary(ctx: &mut ParserContext,) -> Result<Expr, String> {
	let token = ctx.next_token();
	match token {
		Token::StringLit(strlit) => Ok(Expr::StringLit(strlit)),
		Token::IntLit(intlit) => Ok(Expr::Int(intlit)),
		Token::Ident(ident) => Ok(Expr::Ident(ident)),
		Token::OpenParen => {
			let expr = parse_expr(ctx);
			ctx.expect_token(Token::CloseParen)?;
			expr
		},
		_ => Err(format!("Expected expression, got {:?}", token))
	}
}
