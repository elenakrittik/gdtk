use std::iter::Peekable;

use gdtk_lexer::{Token, TokenKind};

pub(crate) fn create_parser(input: &str) -> Peekable<impl Iterator<Item = Token<'_>>> {
    gdtk_lexer::lex(input).peekable()
}

pub(crate) fn next_kind<'a>(iter: &mut Peekable<impl Iterator<Item = Token<'a>>>) -> TokenKind<'a> {
    iter.next().unwrap().kind
}
