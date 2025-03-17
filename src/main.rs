mod tokens;
mod runtime;

use std::env;
use runtime::error;
use std::process::ExitCode;
use std::fs;

fn main() -> ExitCode {
	let args: Vec<String> = env::args().collect(); 
	if args.len() >= 2 {
		for file in args.into_iter().skip(1) {
			let ctx = fs::read_to_string(file).expect("Should have been able to read the file");
			for byte in ctx.chars().as_str().as_bytes() {
				println!("{} = {}", *byte as char, byte);
			}
			// println!("{:?}", ctx.chars().as_str().as_bytes());
		}
	} else {
		return error("you need to give parameters which is files that can readable(raw text).", 1);
	}
	ExitCode::from(0)
}
