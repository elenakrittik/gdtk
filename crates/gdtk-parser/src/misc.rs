use std::iter::Peekable;

use gdtk_ast::poor::{ASTAnnotation, ASTSignal, ASTStatement};
use gdtk_lexer::{Token, TokenKind};

use crate::utils::{collect_values, collect_params, expect_blank_prefixed};

pub fn parse_annotation<'a, T>(iter: &mut Peekable<T>) -> ASTStatement<'a>
where
    T: Iterator<Item = Token<'a>>,
{
    expect_blank_prefixed!(iter, TokenKind::Annotation, ());
    let identifier = expect_blank_prefixed!(iter, TokenKind::Identifier(i), i);
    let arguments = collect_values(
        iter,
        false,
    );

    ASTStatement::Annotation(ASTAnnotation {
        identifier,
        arguments,
    })
}

pub fn parse_signal<'a, T>(iter: &mut Peekable<T>) -> ASTSignal<'a>
where
    T: Iterator<Item = Token<'a>>,
{
    expect_blank_prefixed!(iter, TokenKind::Signal, ());
    let identifier = expect_blank_prefixed!(iter, TokenKind::Identifier(s), s);
    let parameters = collect_params(iter);

    ASTSignal {
        identifier,
        parameters,
    }
}
