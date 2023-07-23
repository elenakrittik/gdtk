pub mod callbacks;
pub mod error;
pub mod indent;
pub mod state;
pub mod token;

use logos::Logos;

pub fn lex(input: &str) -> Result<Vec<token::Token>, error::SpannedError> {
    preprocess(token::Token::lexer(input))
}

fn preprocess<'a>(
    tokens: logos::Lexer<'a, token::Token<'a>>,
) -> Result<Vec<token::Token<'a>>, error::SpannedError> {
    let mut vec = vec![];

    for token in tokens {
        let token = token?;

        vec.push(token);
    }

    Ok(vec)
}
