use std::iter::Peekable;

use gdtk_lexer::Token;

pub fn create_parser<'a>(input: &'a str) -> Peekable<impl Iterator<Item = Token<'a>>> {
    gdtk_lexer::lex(input).0.into_iter().peekable()
}
