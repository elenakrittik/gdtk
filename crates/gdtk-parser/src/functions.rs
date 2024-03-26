use std::iter::Peekable;

use gdtk_ast::poor::{ASTFunction, ASTStatement};
use gdtk_lexer::{Token, TokenKind};

use crate::block::parse_block;
use crate::utils::{expect_blank_prefixed, peek_non_blank};
use crate::variables::parse_variable;

pub fn parse_func<'a, T>(iter: &mut Peekable<T>) -> ASTStatement<'a>
where
    T: Iterator<Item = Token<'a>>,
{
    let identifier = expect_blank_prefixed!(iter, TokenKind::Identifier(s), s);
    let mut parameters = vec![];
    expect_blank_prefixed!(iter, TokenKind::OpeningParenthesis, ());

    loop {
        if !matches!(peek_non_blank!(iter).kind, TokenKind::Identifier(_)) {
            panic!("unexpected {:?}, expected function parameter", iter.next());
        }

        let param = parse_variable(iter, gdtk_ast::poor::ASTVariableKind::FunctionParameter);
        parameters.push(param);

        match peek_non_blank!(iter) {
            Token {
                kind: TokenKind::Comma,
                ..
            } => {
                iter.next();
                continue;
            }
            Token {
                kind: TokenKind::ClosingParenthesis,
                ..
            } => {
                iter.next();
                break;
            }
            other => panic!("unexpected {other:?}, expected a comma or a closing parenthesis"),
        }
    }

    expect_blank_prefixed!(iter, TokenKind::Colon, ());

    let body = parse_block(iter);

    ASTStatement::Func(ASTFunction {
        identifier,
        parameters,
        body,
    })
}
