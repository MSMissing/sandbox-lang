use std::process::exit;

use crate::parser;

pub enum Value {
	_String(String),
	_Int(i64)
}

pub fn eval_expr(expr: parser::Expr) -> Value {
	match expr {
		parser::Expr::StringLit(strlit) => Value::_String(strlit),
		parser::Expr::Int(intlit) => Value::_Int(intlit),
		parser::Expr::SumExpr { sign, summands } => {
			match sign {
				parser::Sign::Add => {
					let Value::_Int(left) = eval_expr(summands[0].clone()) else {unimplemented!()};
					let Value::_Int(right) = eval_expr(summands[1].clone()) else {unimplemented!()};
					Value::_Int(left + right)
				},
				_ => todo!()
			}
		}
		_ => unimplemented!(),
	}
}

pub fn run_code<'a>(nodes: Vec<parser::Node>) -> Result<(), String> {
	let mut current: usize = 0;
	
	while current < nodes.len() {
		match nodes[current].clone() {
			parser::Node::Print(printexpr) => {
				let value = eval_expr(printexpr);
				match value {
					Value::_String(str_to_print) => {
						println!("{}", str_to_print);
					},
					Value::_Int(int_to_print) => {
						println!("{}", int_to_print);
					}
				}
			},
			parser::Node::Exit(exitexpr) => {
				let value = eval_expr(exitexpr);
				match value {
					Value::_Int(exit_code) => {
						println!("Program exited with code {}", exit_code);
						exit(exit_code as i32);
					},
					_ => {
						panic!("Exit code must be an Int");
					}
				}
			}
			// _ => panic!("what da hecc"),
		}
		current += 1;
	}
	
	Ok(())
}
