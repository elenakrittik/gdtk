pub mod callbacks;
pub mod error;
#[cfg(test)] mod tests;
pub mod token;

use crate::error::IntoDiag;
use gdtk_diag::Diagnostic;
use logos::Logos;

pub use crate::token::{Token, TokenKind};

pub type LexOutput<'a> = (Vec<Token<'a>>, Vec<Diagnostic>);

pub fn lex(input: &str) -> LexOutput {
    preprocess(TokenKind::lexer(input))
}

/// Arranges results by their span.
fn preprocess<'a>(lexer: logos::Lexer<'a, TokenKind<'a>>) -> LexOutput<'a> {
    let mut tokens: Vec<Token<'a>> = vec![];
    let mut diags: Vec<Diagnostic> = vec![];

    for (result, span) in lexer.spanned() {
        match result {
            Ok(token) => {
                tokens.push(Token {
                    kind: token,
                    range: span,
                });
            },
            Err(err) => diags.push(err.into_diag(span)),
        }
    }

    (tokens, diags)
}
