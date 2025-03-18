#[derive(Debug)]
#[warn(dead_code)] // LOOK HERE
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
	None,				// none

	Null,				// null pointe
	NewLine				// bir adam yazı yazarken satır başına gelmiş ölmüş xd
}

impl Tokens {
	pub fn lex(ctx: &[u8]) -> Vec<Self> {
		let mut tokens: Vec<Self> = vec![];

		let mut i = 0;
		while i < ctx.len() {
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

				' ' => {
					i += 1;
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
