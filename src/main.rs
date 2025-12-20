mod lexer;
mod parser;
mod interpreter;

use std::env;
use std::fs;

fn main() {
	let args: Vec<String> = env::args().collect();
	
	let file = fs::read_to_string(args[1].clone()).unwrap();
	
	let tokens = lexer::lex(file);
	
	println!("{:?}", &tokens);
	
	let nodes = parser::parse(tokens);
	
	println!("{:?}", &nodes);
	
	match interpreter::run_code(nodes) {
		Ok(_) => {},
		Err(e) => println!("Program failed: {}", e)
	}
}
