use std::iter::Peekable;

use gdtk_ast::poor::ASTStatement;
use gdtk_lexer::{Token, TokenKind};

use crate::utils::expect;
use crate::statement::parse_statement;

pub fn parse_block<'a, T>(iter: &mut Peekable<T>) -> Vec<ASTStatement<'a>>
where
    T: Iterator<Item = Token<'a>>,
{
    let mut stmts = vec![];

    expect!(iter, TokenKind::Newline, ());
    expect!(iter, TokenKind::Indent, ());

    while let Some(token) = iter.next() {
        match token.kind {
            TokenKind::Dedent => break,
            TokenKind::Newline => (),
            _ => stmts.push(parse_statement(iter, Some(token))),
        }
    }

    stmts
}