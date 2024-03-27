use std::iter::Peekable;

use gdtk_ast::poor::CodeBlock;
use gdtk_lexer::{Token, TokenKind};

use crate::statement::parse_statement;
use crate::utils::expect;

pub fn parse_block<'a, T>(iter: &mut Peekable<T>, value: bool) -> CodeBlock<'a>
where
    T: Iterator<Item = Token<'a>>,
{
    let mut stmts = vec![];

    expect!(iter, TokenKind::Newline, ());
    expect!(iter, TokenKind::Indent, ());

    while let Some(Token { kind, .. }) = iter.peek() {
        match kind {
            &TokenKind::Dedent => {
                iter.next();
                break;
            },
            &TokenKind::Newline => {
                iter.next();
            },
            &TokenKind::ClosingParenthesis
            | &TokenKind::ClosingBracket
            | &TokenKind::ClosingBrace => {
                if value {
                    break;
                } else {
                    let token = iter.next();
                    stmts.push(parse_statement(iter, token));
                }
            }
            _ => {
                let token = iter.next();
                stmts.push(parse_statement(iter, token));
            },
        }
    }

    stmts
}
