#[cfg(test)]
mod tests {

	use crate::interpreter::{Interpreter, run_code};
	use crate::lexer::{LexerContext, lex};
	use crate::parser::{ParserContext, parse};

	macro_rules! process {
		( $code:literal, $expected_exit:expr, $expected_output:literal ) => {{
			let tokens = lex(&mut LexerContext::new(String::from($code))).unwrap();
			let nodes = parse(&mut ParserContext::new(tokens), 0).unwrap();
			let mut ctx = Interpreter::new();
			run_code(&mut ctx, nodes).unwrap();
			assert!(ctx.exit_code.or(Some(0)).unwrap() == $expected_exit);
			assert!(*ctx.output == String::from($expected_output));
		}};
	}

	fn expect_error(code: String) -> Result<String, ()> {
		let tokens = lex(&mut LexerContext::new(code));
		match tokens {
			Err(e) => Ok(e),
			Ok(o) => {
				let nodes = parse(&mut ParserContext::new(o), 0);
				match nodes {
					Ok(o) => {
						let exit_code = run_code(&mut Interpreter::new(), o);
						match exit_code {
							Ok(o) => Err(o),
							Err(e) => Ok(e),
						}
					}
					Err(e) => Ok(e),
				}
			}
		}
	}

	#[test]
	fn basic() {
		process!("exit 24", 24, "");
	}

	#[test]
	fn arithmetic() {
		process!("exit 1 + 2 * 3 / 4", 1 + 2 * 3 / 4, "");
	}

	#[test]
	fn while_loop() {
		process!(
			"x:=5 y:=0 while x {print x y+=x x-=1}exit y",
			5 + 4 + 3 + 2 + 1,
			"5\n4\n3\n2\n1\n"
		);
	}

	#[test]
	fn strings() {
		process!("print \"Hello, world!\"", 0, "Hello, world!\n");
	}

	#[test]
	fn types() {
		expect_error(String::from("x: Int = true")).unwrap();
	}
}
