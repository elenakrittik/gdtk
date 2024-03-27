use std::iter::Peekable;

use gdtk_ast::poor::{ASTVariable, ASTVariableKind};
use gdtk_lexer::{Token, TokenKind};

use crate::utils::{expect_blank_prefixed, next_non_blank, peek_non_blank};
use crate::values::parse_value;

pub fn parse_variable<'a, T>(iter: &mut Peekable<T>, kind: ASTVariableKind) -> ASTVariable<'a>
where
    T: Iterator<Item = Token<'a>>,
{
    let identifier = expect_blank_prefixed!(iter, TokenKind::Identifier(s), s);
    let mut typehint = None;
    let mut infer_type = false;
    let mut value = None;

    // Possible cases:
    // [var] ident
    // [var] ident = val
    // [var] ident := val
    // [var] ident: type = val
    // [var] ident: type

    if matches!(
        peek_non_blank!(iter).kind,
        TokenKind::Colon | TokenKind::Assignment
    ) {
        match next_non_blank!(iter) {
            Token {
                kind: TokenKind::Colon,
                ..
            } => match next_non_blank!(iter) {
                Token {
                    kind: TokenKind::Assignment,
                    ..
                } => {
                    infer_type = true;
                    value = Some(parse_value(iter, None));
                }
                other => {
                    typehint = Some(parse_value(iter, Some(other)));

                    if matches!(peek_non_blank!(iter).kind, TokenKind::Assignment) {
                        match next_non_blank!(iter) {
                            Token {
                                kind: TokenKind::Assignment,
                                ..
                            } => value = Some(parse_value(iter, None)),
                            _ => unreachable!(),
                        }
                    }
                }
            },
            Token {
                kind: TokenKind::Assignment,
                ..
            } => value = Some(parse_value(iter, None)),
            _ => unreachable!(),
        }
    }

    ASTVariable {
        identifier,
        infer_type,
        typehint,
        value,
        kind,
    }
}
