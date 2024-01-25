use gdtk_ast::poor::{ASTVariable, ASTVariableKind};
use gdtk_lexer::{Token, TokenKind};

use crate::utils::{expect_blank_prefixed, next_non_blank, parse_idtydef};
use crate::values::parse_value;

pub fn parse_const<'a, T>(iter: &mut T) -> ASTVariable<'a>
where
    T: Iterator<Item = Token<'a>>,
{
    let identifier = expect_blank_prefixed!(iter, TokenKind::Identifier(s), s);

    let mut typehint = None;
    let mut infer_type = false;

    // either colon or an assignment
    let value = match next_non_blank!(iter) {
        // got a colon, has to be followed by an identifier (type hint) or an assignment
        Token {
            kind: TokenKind::Colon,
            ..
        } => {
            match next_non_blank!(iter) {
                Token {
                    kind: TokenKind::Identifier(s),
                    ..
                } => {
                    typehint = Some(s);

                    expect_blank_prefixed!(iter, TokenKind::Assignment, ());
                    parse_value(iter, None)
                }
                // infer type
                Token {
                    kind: TokenKind::Assignment,
                    ..
                } => {
                    infer_type = true;
                    parse_value(iter, None)
                }
                other => panic!("unexpected {other:?}, expected identifier or assignment"),
            }
        }
        Token {
            kind: TokenKind::Assignment,
            ..
        } => parse_value(iter, None),
        other => panic!("unexpected {other:?}, expected colon or assignment"),
    };

    ASTVariable {
        identifier,
        infer_type,
        typehint,
        value: Some(value),
        kind: ASTVariableKind::Constant,
    }
}

pub fn parse_var<'a, T>(iter: &mut T) -> ASTVariable<'a>
where
    T: Iterator<Item = Token<'a>>,
{
    let (identifier, infer_type, typehint, value) = parse_idtydef!(iter, TokenKind::Newline => (),);

    ASTVariable {
        identifier,
        infer_type,
        typehint,
        value,
        kind: ASTVariableKind::Regular,
    }
}
