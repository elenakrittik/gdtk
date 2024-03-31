use std::iter::Peekable;

use gdtk_lexer::Token;

pub fn create_parser(input: &str) -> Peekable<impl Iterator<Item = Token<'_>>> {
    gdtk_lexer::lex(input).0.into_iter().peekable()
}
