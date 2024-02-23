use std::iter::Peekable;

use gdtk_ast::poor::ASTStatement;
use gdtk_lexer::{Token, TokenKind};

use crate::statement::parse_statement;
use crate::utils::expect;

pub fn parse_block<'a, T>(iter: &mut Peekable<T>) -> Vec<ASTStatement<'a>>
where
    T: Iterator<Item = Token<'a>>,
{
    let mut stmts = vec![];

    match iter.next().unwrap().kind {
        TokenKind::Newline => expect!(iter, TokenKind::Indent, ()),
        TokenKind::Indent => (),
        _ => panic!("expected TokenKind::Indent | TokenKind::Newline"),
    }

    while let Some(token) = iter.next() {
        match token.kind {
            TokenKind::Dedent => break,
            TokenKind::Newline => (),
            _ => stmts.push(parse_statement(iter, Some(token))),
        }
    }

    stmts
}
