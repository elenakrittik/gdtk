
use gdtk_lexer::{Token, TokenKind};

use crate::Parser;

pub(crate) fn create_parser(input: &str) -> Parser<impl Iterator<Item = Token<'_>>> {
    crate::parser::Parser::new(gdtk_lexer::lex(input))
}

pub(crate) fn next_kind<'a>(iter: &mut Parser<impl Iterator<Item = Token<'a>>>) -> TokenKind<'a> {
    iter.next().unwrap().kind
}
