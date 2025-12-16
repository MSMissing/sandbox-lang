use crate::lexer;

#[derive(Debug)]
pub enum Expr<'a> {
	BinExpr(&'a Expr<'a>, &'a Expr<'a>),
	StringLit(String),
	Int(i64),
}

#[derive(Debug)]
pub enum Node<'a> {
	Print(Expr<'a>)
}

fn expect_token(i: &mut usize, tokens: &Vec<lexer::Token>, token: lexer::Token) -> Result<lexer::Token, String> {
	if check_token(i, tokens, token.clone()) {
		return Ok(tokens[*i - 1].clone());
	} else {
		return Err(format!("Expected {:?}, but got {:?}", token, tokens[*i]));
	}
}

fn check_token(i: &mut usize, tokens: &Vec<lexer::Token>, token: lexer::Token) -> bool {
	if tokens[*i].clone() == token {
		*i += 1;
		return true;
	}
	return false;
}

fn parse_expr<'a>(i: &mut usize, tokens: &Vec<lexer::Token>) -> Expr<'a> {
	*i += 1;
	match tokens[*i - 1].clone() {
		lexer::Token::StringLit(strlit) => Expr::StringLit(strlit),
		_ => panic!("Expected expression, got {:?}", tokens[*i])
	}
}

pub fn parse<'a>(tokens: Vec<lexer::Token>) -> Vec<Node<'a>> {
	let mut nodes = Vec::<Node<'a>>::new();
	
	let mut i: usize = 0;
	while i < tokens.len() {
		nodes.push(match tokens[i] {
			lexer::Token::Print => {
				i += 1;
				expect_token(&mut i, &tokens, lexer::Token::OpenParen).unwrap();
				let expr = parse_expr(&mut i, &tokens);
				expect_token(&mut i, &tokens, lexer::Token::CloseParen).unwrap();
				Node::Print(expr)
			}
			_ => {
				panic!("Unexpected token {:?}", tokens[i]);
			}
		});
	}
	
	return nodes;
}
