pub mod callbacks;
pub mod error;
pub mod token;
#[cfg(test)]
mod tests;

use error::WithSpan;
use logos::Logos;

pub type Lexeme<'a> = Result<(token::Token<'a>, logos::Span), (error::Error, logos::Span)>;
pub type LexOutput<'a> = Vec<Lexeme<'a>>;

pub fn lex(input: &str) -> LexOutput {
    preprocess(token::Token::lexer(input))
}

/// Arranges results by their span.
fn preprocess<'a>(
    lexer: logos::Lexer<'a, token::Token<'a>>,
) ->  LexOutput {
    let mut vec = vec![];

    for (token, span) in lexer.spanned() {
        vec.push(token
            .map_err(|e| e.with_span(span.clone())) // hopefully not as slow
            .map(|v| v.with_span(span))
        );
    }

    vec.sort_by(|a: _, b: _| {
        let a = match a {
            Ok(tkn) => &tkn.1,
            Err(e) => &e.1,
        };

        let b = match b {
            Ok(tkn) => &tkn.1,
            Err(e) => &e.1,
        };

        a.start.partial_cmp(&b.start).unwrap()
    });

    vec
}
