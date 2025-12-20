
use crate::lexer::Token;

#[derive(Debug, Clone)]
pub enum Expr {
	SumExpr {
		sign: Sign,
		summands: Vec<Expr>
	},
	StringLit(String),
	Int(i64),
	Ident(String),
}

#[derive(Debug, Clone)]
pub enum Sign {
	Add,
	Subtract,
	Multiply,
	Divide,
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


fn expect_token(i: &mut usize, tokens: &Vec<Token>, token: Token) -> Result<Token, String> {
	if check_token(i, tokens, token.clone()) {
		*i += 1;
		return Ok(tokens[*i - 1].clone());
	} else {
		return Err(format!("Expected {:?}, but got {:?}", token, tokens[*i]));
	}
}

fn check_token(i: &usize, tokens: &Vec<Token>, token: Token) -> bool {
	if tokens[*i].clone() == token {
		return true;
	}
	return false;
}

fn parse_expr(i: &mut usize, tokens: &Vec<Token>) -> Result<Expr, String> {
	let expr = match tokens[*i].clone() {
		Token::StringLit(strlit) => Expr::StringLit(strlit),
		Token::IntLit(intlit) => Expr::Int(intlit),
		Token::Ident(ident) => Expr::Ident(ident),
		_ => panic!("Expected expression, got {:?}", tokens[*i])
	};
	*i += 1;
	if *i >= tokens.len() {
		return Ok(expr);
	}
	match tokens[*i] {
		Token::Plus|Token::Dash|Token::Star|Token::Slash => {
			let sign = match tokens[*i] {
				Token::Plus => Ok(Sign::Add),
				Token::Dash => Ok(Sign::Subtract),
				Token::Star => Ok(Sign::Multiply),
				Token::Slash => Ok(Sign::Divide),
				_ => Err(format!("Expected sign, but got {:?}, this should never happen.", tokens[*i])),
			}?;
			let sum = Expr::SumExpr { sign: sign, summands: {
				*i += 1;
				vec!(expr, parse_expr(i, tokens)?)
			}};
			
			Ok(sum)
		},
		_ => Ok(expr)
	}
}

pub fn parse<'a>(tokens: Vec<Token>) -> Result<Vec<Node>, String> {
	let mut nodes = Vec::<Node>::new();
	
	let mut i: usize = 0;
	while i < tokens.len() {
		let node: Result<Node, String> = match tokens[i].clone() {
			Token::Print => {
				i += 1;
				expect_token(&mut i, &tokens, Token::OpenParen)?;
				let expr = parse_expr(&mut i, &tokens)?;
				expect_token(&mut i, &tokens, Token::CloseParen)?;
				Ok(Node::Print(expr))
			},
			Token::Exit => {
				i += 1;
				expect_token(&mut i, &tokens, Token::OpenParen)?;
				let expr = parse_expr(&mut i, &tokens)?;
				expect_token(&mut i, &tokens, Token::CloseParen)?;
				Ok(Node::Exit(expr))
			},
			Token::Ident(ident) => {
				i += 1;
				match tokens[i] {
					Token::Colon => {
						i += 1;
						expect_token(&mut i, &tokens, Token::Equals)?;
						let expr = parse_expr(&mut i, &tokens)?;
						Ok(Node::Let {ident: ident, expr: expr})
					},
					Token::Equals => {
						i += 1;
						Ok(Node::Assign { ident, expr: parse_expr(&mut i, &tokens)? })
					},
					_ => Err(format!("Expected : or =, but got {:?}", tokens[i]))
				}
			}
			_ => Err(format!("Unexpected token {:?}", tokens[i]))
		};
		nodes.push(node?);
	}
	
	return Ok(nodes);
}
