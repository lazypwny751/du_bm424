#[derive(Debug)]
pub(crate) enum TokenValue {
	Plus,				// +
	Minus,				// -

	If,					// if
	Elif, // buna pek gerek yok lakin yengenin adı elifse kullanırsın knk yoksa bilidğin "else if"
	Else,				// else
	LeftCurlyBracet, 	// {
	RightCurlBracet,	// }
	LeftBracet, 		// (
	RightBracet, 		// )

	Num(i32),			// sayı
	Text(String),		// bu string literal olacak, çift tırnak arasına giren her şey.
	True,				// true
	False,				// false
}

#[derive(Debug)]
pub struct Token {
	pub token: String,
	pub value: TokenValue,
}

impl Token {
	pub fn lex(ctx: &[u8]) -> Vec<Self> {
		let tokens: Vec<Self> = vec![];
		// for byte in ctx {
			// match byte {
				// 10 => {
					// 
				// }
			// }
		// }
		tokens
	}
}
