use gdtk_ast::poor::ASTStatement;
use gdtk_lexer::{Token, TokenKind};

use crate::utils::expect;

pub fn parse_block<'a, T>(iter: &mut T) -> Vec<ASTStatement<'a>>
where
    T: Iterator<Item = Token<'a>>,
{
    expect!(iter, TokenKind::Newline, ());

    vec![]
}
