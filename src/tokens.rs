#[derive(Debug)]
#[allow(dead_code)] // LOOK HERE
pub(crate) enum Tokens {
	Plus,				// +
	Minus,				// -

	If,					// if
	Elif, // buna pek gerek yok lakin yengenin adı elifse kullanırsın knk yoksa bilidğin "else if"
	Else,				// else
	LeftCurlyBracet, 	// {
	RightCurlBracet,	// }
	LeftBracet, 		// (
	RightBracet, 		// )

	Number(String),		// sayı, evet sayı nasıl string değer alıyor diyon ama lexer için her şey char.
	Text(String),		// bu string literal olacak, çift tırnak arasına giren her şey.
	True,				// true
	False,				// false

	Null,				// null pointe
	None,				// nothing to parse
	NewLine				// bir adam yazı yazarken satır başına gelmiş ölmüş xd
}

impl Tokens {
	pub fn lex(ctx: &[u8]) -> Vec<Self> {
		let mut tokens: Vec<Self> = vec![];
		let mut nlvl: u32 = 0; // bu hep 0 olacak.

		let mut i = 0;
		while i < ctx.len() {
			// println!("{:?}", ctx[i] as char);
			let mut buf: String = String::new();

			match ctx[i] as char {
				'0'..'9' => {
					while (ctx[i] as char) >= '0' && (ctx[i] as char) <= '9' {
						buf.push(ctx[i] as char);
						i += 1;
					}
					tokens.push(Tokens::Number(buf));
				},

				'\n' => {
					tokens.push(Tokens::NewLine);
				},

				' ' => {},

				'(' => {
					tokens.push(Tokens::LeftBracet);
					nlvl += 1;
				},

				')' => {
					tokens.push(Tokens::RightBracet);
					nlvl -= 1;	
				},

				'{' => {
					tokens.push(Tokens::LeftCurlyBracet);
					nlvl += 1;
				},

				'}' => {
					tokens.push(Tokens::RightCurlBracet);
					nlvl -= 1;
				},

				_ => {
					buf.push(ctx[i] as char);
					tokens.push(Tokens::None);
				}
			}
			i += 1;
		}

		tokens
	}
}
