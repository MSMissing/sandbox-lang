use crate::lexer;

#[derive(Debug, Clone)]
pub enum Expr {
	SumExpr {
		sign: Sign,
		summands: Vec<Expr>
	},
	StringLit(String),
	Int(i64),
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
}

fn expect_token(i: &mut usize, tokens: &Vec<lexer::Token>, token: lexer::Token) -> Result<lexer::Token, String> {
	if check_token(i, tokens, token.clone()) {
		*i += 1;
		return Ok(tokens[*i - 1].clone());
	} else {
		return Err(format!("Expected {:?}, but got {:?}", token, tokens[*i]));
	}
}

fn check_token(i: &usize, tokens: &Vec<lexer::Token>, token: lexer::Token) -> bool {
	if tokens[*i].clone() == token {
		return true;
	}
	return false;
}

fn parse_expr(i: &mut usize, tokens: &Vec<lexer::Token>) -> Expr {
	let expr = match tokens[*i].clone() {
		lexer::Token::StringLit(strlit) => Expr::StringLit(strlit),
		lexer::Token::IntLit(intlit) => Expr::Int(intlit),
		_ => panic!("Expected expression, got {:?}", tokens[*i])
	};
	*i += 1;
	match tokens[*i] {
		lexer::Token::Plus|lexer::Token::Dash|lexer::Token::Star|lexer::Token::Slash => {
			let sum = Expr::SumExpr { sign: match tokens[*i] {
				lexer::Token::Plus => Sign::Add,
				lexer::Token::Dash => Sign::Subtract,
				lexer::Token::Star => Sign::Multiply,
				lexer::Token::Slash => Sign::Divide,
				_ => panic!("Expected sign, but got {:?}, this should never happen.", tokens[*i]),
			}, summands: {
				*i += 1;
				vec!(expr, parse_expr(i, tokens))
			}};
			
			return sum;
		},
		_ => {return expr;}
	};
}

pub fn parse<'a>(tokens: Vec<lexer::Token>) -> Vec<Node> {
	let mut nodes = Vec::<Node>::new();
	
	let mut i: usize = 0;
	while i < tokens.len() {
		nodes.push(match tokens[i] {
			lexer::Token::Print => {
				i += 1;
				expect_token(&mut i, &tokens, lexer::Token::OpenParen).unwrap();
				let expr = parse_expr(&mut i, &tokens);
				expect_token(&mut i, &tokens, lexer::Token::CloseParen).unwrap();
				Node::Print(expr)
			},
			lexer::Token::Exit => {
				i += 1;
				expect_token(&mut i, &tokens, lexer::Token::OpenParen).unwrap();
				let expr = parse_expr(&mut i, &tokens);
				expect_token(&mut i, &tokens, lexer::Token::CloseParen).unwrap();
				Node::Exit(expr)
			}
			_ => {
				panic!("Unexpected token {:?}", tokens[i]);
			}
		});
	}
	
	return nodes;
}
