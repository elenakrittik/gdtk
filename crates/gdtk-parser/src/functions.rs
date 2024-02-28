use std::iter::Peekable;

use gdtk_ast::poor::{ASTStatement, ASTFunction, ASTFunctionParameter};
use gdtk_lexer::{Token, TokenKind};

use crate::block::parse_block;
use crate::utils::{expect_blank_prefixed, next_non_blank, parse_idtydef};

pub fn parse_func<'a, T>(iter: &mut Peekable<T>) -> ASTStatement<'a>
where
    T: Iterator<Item = Token<'a>>,
{
    let identifier = expect_blank_prefixed!(iter, TokenKind::Identifier(s), s);
    expect_blank_prefixed!(iter, TokenKind::OpeningParenthesis, ());
    let mut parameters = vec![];

    loop {
        if matches!(
            iter.peek(),
            Some(Token {
                kind: TokenKind::ClosingParenthesis,
                ..
            })
        ) {
            iter.next();
            break;
        }

        let mut expect_comma = true;
        let mut break_ = false;

        let (identifier, infer_type, typehint, default) = parse_idtydef!(
            iter,
            TokenKind::Comma => { dbg!("got comma"); expect_comma = false; },
            TokenKind::ClosingParenthesis => { break_ = true; dbg!("got end paren"); },
        );

        parameters.push(ASTFunctionParameter {
            identifier,
            infer_type,
            typehint,
            default,
        });

        if break_ {
            break;
        }

        if expect_comma {
            match next_non_blank!(iter) {
                Token {
                    kind: TokenKind::Comma,
                    ..
                } => (),
                Token {
                    kind: TokenKind::ClosingParenthesis,
                    ..
                } => break,
                other => panic!("expected comma or closing parenthesis, found {other:?}"),
            }
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
