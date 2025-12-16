use std::env;
use std::fs;

fn main() {
	let args: Vec<String> = env::args().collect();
	
	let file = fs::read_to_string(args[1].clone()).unwrap();
	
	println!("{}", file);
}
