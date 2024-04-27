mod cursor;
mod lexer;
mod symbols;
#[cfg(test)]
mod tests;
mod token;

use crate::lexer::lexer::Lexer;
pub use crate::lexer::token::{Token, TokenKind};

pub fn lex(input: &str) -> impl Iterator<Item = Token<'_>> {
    Lexer::new(input)
}
