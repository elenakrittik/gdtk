mod cursor;
mod lexer;
mod symbols;
mod blankets;
mod identifier;
mod number;
#[cfg(test)]
mod tests;
mod token;

use crate::lexer::lexer::Lexer;
pub use crate::lexer::token::{Token, TokenKind};

pub fn lex(input: &str) -> impl Iterator<Item = Token<'_>> {
    Lexer::new(input)
}
