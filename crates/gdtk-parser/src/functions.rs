use gdtk_lexer::Token;
use gdtk_ast::poor::{ASTFunction, ASTFunctionParameter, CodeBlock};

use crate::utils::{expect_blank_prefixed, next_non_blank, parse_idtydef};

pub fn parse_func<'a, T>(iter: &mut T) -> ASTFunction<'a>
where
    T: Iterator<Item = Token<'a>>,
{
    let identifier = expect_blank_prefixed!(iter, Token::Identifier(s), s);
    expect_blank_prefixed!(iter, Token::OpeningParenthesis, ());
    let mut parameters = vec![];

    loop {
        let mut expect_comma = true;
        let mut break_ = false;

        let (identifier, infer_type, typehint, default) = parse_idtydef!(
            iter,
            Token::Comma => { dbg!("got comma"); expect_comma = false; },
            Token::ClosingParenthesis => { break_ = true; dbg!("got end paren"); },
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
                Token::Comma => (),
                Token::ClosingParenthesis => break,
                other => panic!("expected comma or closing parenthesis, found {other:?}"),
            }
        }
    }

    expect_blank_prefixed!(iter, Token::Colon, ());

    ASTFunction {
        identifier,
        parameters,
        body: vec![],
    }
}

pub fn parse_func_body<'a, T>(iter: &mut T) -> CodeBlock<'a>
where
    T: Iterator<Item = Token<'a>>,
{
    vec![]
}
