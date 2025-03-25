mod tokens;
mod runtime;

use std::env;
use runtime::error;
use std::process::ExitCode;
use std::fs;
use tokens::Tokens;

fn main() -> ExitCode {
	let args: Vec<String> = env::args().collect();

	if args.len() >= 2 {
		for file in args.into_iter().skip(1) {
			let ctx = fs::read_to_string(file).expect("Failed to read file.");
			let bytes = ctx.chars().as_str().as_bytes();

			for token in Tokens::lex(&bytes) {
				println!("{:?}", token);
			}
		}
	} else {
		return error("Need's a file path as parameter.", 1);
	}
	ExitCode::from(0)
}
