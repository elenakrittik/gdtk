use gdtk_ast::poor::{ASTFunction, ASTFunctionParameter, CodeBlock};
use gdtk_lexer::{Token, TokenKind};

use crate::utils::{expect_blank_prefixed, next_non_blank, parse_idtydef};

pub fn parse_func<'a, T>(iter: &mut T) -> ASTFunction<'a>
where
    T: Iterator<Item = Token<'a>>,
{
    let identifier = expect_blank_prefixed!(iter, TokenKind::Identifier(s), s);
    expect_blank_prefixed!(iter, TokenKind::OpeningParenthesis, ());
    let mut parameters = vec![];

    loop {
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

    ASTFunction {
        identifier,
        parameters,
        body: vec![],
    }
}

pub fn parse_func_body<'a, T>(_iter: &mut T) -> CodeBlock<'a>
where
    T: Iterator<Item = Token<'a>>,
{
    vec![]
}
