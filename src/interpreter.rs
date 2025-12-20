use std::{collections::HashMap, process::exit};

use crate::parser::{Node, Expr, Sign};

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub enum Value {
	_String(String),
	_Int(i64)
}

pub struct Interpreter {
	variables: HashMap<String, Value>
}

impl Interpreter {
	fn new() -> Self {
		return Interpreter {
			variables: HashMap::new()
		}
	}
}


pub fn eval_expr(expr: Expr, ctx: &Interpreter) -> Result<Value, String> {
	match expr {
		Expr::StringLit(strlit) => Ok(Value::_String(strlit)),
		Expr::Int(intlit) => Ok(Value::_Int(intlit)),
		Expr::SumExpr { sign, summands } => {
			let Value::_Int(left) = eval_expr(summands[0].clone(), ctx)? else {unimplemented!()};
			let Value::_Int(right) = eval_expr(summands[1].clone(), ctx)? else {unimplemented!()};
			match sign {
				Sign::Add => {
					Ok(Value::_Int(left + right))
				},
				Sign::Subtract => {
					Ok(Value::_Int(left - right))
				}
				_ => todo!()
			}
		},
		Expr::Ident(ident) => {
			match ctx.variables.get(&ident) {
				Some(val) => Ok((*val).clone()),
				None => Err(format!("Variable {} is uninitialized", ident))
			}
		}
		//_ => unimplemented!(),
	}
}


pub fn run_code(nodes: Vec<Node>) -> Result<(), String> {
	let mut current: usize = 0;
	let mut ctx = Interpreter::new();
	
	while current < nodes.len() {
		match nodes[current].clone() {
			Node::Print(printexpr) => {
				let value = eval_expr(printexpr, &ctx)?;
				match value {
					Value::_String(str_to_print) => {
						println!("{}", str_to_print);
					},
					Value::_Int(int_to_print) => {
						println!("{}", int_to_print);
					}
				}
			},
			Node::Exit(exitexpr) => {
				let value = eval_expr(exitexpr, &ctx)?;
				match value {
					Value::_Int(exit_code) => {
						println!("Program exited with code {}", exit_code);
						exit(exit_code as i32);
					},
					_ => {
						panic!("Exit code must be an Int");
					}
				}
			},
			Node::Let { ident, expr } => {
				ctx.variables.insert(ident.clone(), eval_expr(expr, &ctx)?);
			},
			Node::Assign { ident, expr } => {
				let value = eval_expr(expr, &ctx)?;
				match ctx.variables.get_mut(&ident) {
					Some(var) => {
						*var = value;
					},
					None => panic!("{} not defined", ident)
				}
			}
			// _ => panic!("what da hecc"),
		}
		current += 1;
	}
	
	Ok(())
}
