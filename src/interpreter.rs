use std::mem::discriminant;
use std::process::exit;
use std::collections::HashMap;

use crate::expr::{Sign, Expr, Type};
use crate::parser::{Node};

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub enum Value {
	_String(String),
	_Int(i64),
	_Bool(bool)
}

pub struct Scope {
	variables: HashMap<String, Variable>
}

impl Scope {
	pub fn new() -> Self {
		Scope {
			variables: HashMap::new()
		}
	}
}


#[derive(Clone, Debug)]
pub struct Variable {
	value: Value,
	var_type: Type
}

impl Variable {
	pub fn new(value: Value, var_type: Type) -> Self {
		Self { value, var_type }
	}
}

pub struct Interpreter {
	scopes: Vec<Scope>
}

impl Interpreter {
	pub fn new() -> Self {
		return Interpreter {
			scopes: Vec::new(),
		}
	}
	pub fn get_scope(self: &mut Self) -> &mut Scope {
		return self.scopes.last_mut().unwrap()
	}
	pub fn get_var(self: &Self, ident: String) -> Result<&Variable, String> {
		let scope_len = self.scopes.len();
		for i in 0..scope_len {
			match self.scopes[(scope_len - 1) - i].variables.get(&ident) {
				Some(var) => return Ok(var),
				None => ()
			};
		}
		Err(format!("Variable {} is not initialized.", ident))
	}
	pub fn set_var(self: &mut Self, ident: String, value: Value) -> Result<(), String> {
		let value_type = Type::from_value(value.clone());
		let scope_len = self.scopes.len();
		for i in 0..scope_len {
			match self.scopes[(scope_len - 1) - i].variables.get_mut(&ident) {
				Some(var) => {
					if discriminant(&var.var_type) != discriminant(&value_type) {
						return Err(format!("Implicit conversion from {:?} to {:?} is not supported", var.var_type, value_type))
					}
					var.value = value;
					return Ok(());
				},
				None => ()
			};
		}
		return Err(format!("Could not set the value of {}", ident));
	}
	pub fn init_var(self: &mut Self, ident: String, value: Value, var_type: Type) -> Result<(), String> {
		self.get_scope().variables.insert(ident, Variable::new(value.clone(), match var_type == Type::Auto {
			true => Ok(Type::from_value(value)),
			false => {
				let expr_type = Type::from_value(value);
				if expr_type != var_type {
					Err(format!("Expected type {:?}, but got {:?}", var_type, expr_type))
				} else {
					Ok(var_type)
				}
			}
		}?));
		Ok(())
	}
}


pub fn eval_expr(expr: Expr, ctx: &Interpreter) -> Result<Value, String> {
	match expr {
		Expr::StringLit(strlit) => Ok(Value::_String(strlit)),
		Expr::Int(intlit) => Ok(Value::_Int(intlit)),
		Expr::Bool(boollit) => Ok(Value::_Bool(boollit)),
		Expr::Not(expr2) => {
			let Value::_Bool(value) = eval_expr(*expr2, ctx)? else {
				return Err(format!("Expected bool after !"));
			};
			
			Ok(Value::_Bool(!value))
		}
		
		Expr::SumExpr { sign, summands } => {
			let left  = eval_expr(summands[0].clone(), ctx)?;
			let right = eval_expr(summands[1].clone(), ctx)?;
			
			if sign == Sign::Equal {
				return Ok(Value::_Bool(left == right));
			} else if discriminant(&left) == discriminant(&right) {
				match (left, right) {
					(Value::_String(lstr), Value::_String(rstr)) => {
						match sign {
							Sign::Concat => Ok(Value::_String(lstr + &rstr)),
							_ => Err(format!("Cannot use sign {:?} on Strings.", sign))
						}
					},
					(Value::_Int(lint), Value::_Int(rint)) => {
						match sign {
							Sign::Add      => Ok(Value::_Int(lint + rint)),
							Sign::Subtract => Ok(Value::_Int(lint - rint)),
							Sign::Multiply => Ok(Value::_Int(lint * rint)),
							Sign::Divide   => Ok(Value::_Int(lint / rint)),
							_ => Err(format!("Cannot use sign {:?} on Ints.", sign))
						}
					},
					(Value::_Bool(lbool), Value::_Bool(rbool)) => {
						match sign {
							Sign::Equal => Ok(Value::_Bool(lbool == rbool)),
							_ => Err(format!("Cannot use sign {:?} on Bools", sign)),
						}
					},
					_ => panic!("This should never happen.")
				}
			} else {
				Err("Implicit type casting not implemented".to_string())
			}
		},
		
		Expr::Ident(ident) => {
			Ok(ctx.get_var(ident)?.value.clone())
		}
		//_ => unimplemented!(),
	}
}

fn truthy(value: Value) -> bool {
	match value {
		 Value::_String(s) => {
			 !s.is_empty()
		},
		Value::_Int(i) => {
			i != 0
		},
		Value::_Bool(b) => {
			b
		}
	}
}

pub fn run_code(ctx: &mut Interpreter, nodes: Vec<Node>) -> Result<(), String> {
	ctx.scopes.push(Scope::new());
	let mut current: usize = 0;
	
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
					},
					Value::_Bool(bool_to_print) => {
						println!("{}", bool_to_print);
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
					Value::_Bool(exit_code) => {
						exit(!exit_code as i32);
					}
					_ => {
						panic!("Exit code must be an Int");
					}
				}
			},
			Node::Let { ident, expr, var_type } => {
				ctx.init_var(ident.clone(), eval_expr(expr, ctx)?, var_type)?;
			},
			Node::Assign { ident, expr } => {
				let value = eval_expr(expr, ctx)?;
				ctx.set_var(ident, value)?;
			},
			Node::Scope(nodes) => {
				run_code(ctx, nodes)?;
			},
			Node::If { expr, body } => {
				let value = eval_expr(expr, ctx)?;
				let Node::Scope(body_nodes) = *body else {panic!("Expected Node::Scope for if body. This should never happen.");};
				if truthy(value) {
					run_code(ctx, body_nodes)?;
				}
			},
			Node::While { expr, body } => {
				let mut value = eval_expr(expr.clone(), ctx)?;
				let Node::Scope(body_nodes) = *body else {panic!("Expected Node::Scope for if body. This should never happen.");};
				while truthy(value.clone()) {
					run_code(ctx, body_nodes.clone())?;
					value = eval_expr(expr.clone(), ctx)?;
				}
			}
			// _ => panic!("what da hecc"),
		};
		current += 1;
	}
	
	ctx.scopes.pop();
	Ok(())
}
