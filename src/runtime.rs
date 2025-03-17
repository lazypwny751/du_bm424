use std::process::ExitCode;

pub fn error(text: &str, ret: u8) -> ExitCode {
	eprintln!("error: {}", text);
	ExitCode::from(ret)
}
