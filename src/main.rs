mod expr;
mod interpreter;
mod lexer;
mod parser;
mod tests;

use std::fs;

use clap::Parser;

use crate::{interpreter::Interpreter, lexer::LexerContext, parser::ParserContext};

#[derive(Parser, Debug)]
#[command(author = "msMissing", version, about, long_about = None)]
struct Arguments {
	#[arg(short, long)]
	file: String,
}

fn main() -> Result<(), String> {
	let args = Arguments::parse();

	let file = fs::read_to_string(args.file.clone()).unwrap();

	let tokens = lexer::lex(&mut LexerContext::new(file))?;

	println!("TOKENS: {:?}", &tokens);

	let mut parser_ctx = ParserContext::new(tokens);

	let nodes = parser::parse(&mut parser_ctx, 0)?;

	println!("NODES: {:?}", &nodes);

	let mut interpreter_ctx = Interpreter::new();

	println!();

	interpreter::run_code(&mut interpreter_ctx, nodes)?;

	println!(
		"Exit code: {}",
		interpreter_ctx.exit_code.unwrap_or(0)
	);
	Ok(())
}
