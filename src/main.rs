mod lexer;
mod parser;
mod interpreter;
mod expr;

use std::fs;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author = "msMissing", version, about, long_about = None)]
struct Arguments {
	#[arg(short, long)]
	file: Option<String>
}

fn main() -> Result<(), String> {
	let args = Arguments::parse();
	
	let file = fs::read_to_string(args.file.unwrap().clone()).unwrap();
	
	let tokens = lexer::lex(file);
	
	println!("{:?}", &tokens);
	
	let nodes = parser::parse(tokens)?;
	
	println!("{:?}", &nodes);
	
	interpreter::run_code(nodes)
}
