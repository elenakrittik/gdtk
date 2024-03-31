use std::iter::Peekable;

use gdtk_ast::poor::{ASTAnnotation, ASTSignal, ASTStatement, ASTVariableKind};
use gdtk_lexer::{Token, TokenKind};

use crate::{expressions::parse_expr, utils::{delemited_by, expect_blank_prefixed}, variables::parse_variable_body};

pub fn parse_annotation<'a>(iter: &mut Peekable<impl Iterator<Item = Token<'a>>>) -> ASTStatement<'a> {
    expect_blank_prefixed!(iter, TokenKind::Annotation, ());

    let identifier = expect_blank_prefixed!(iter, TokenKind::Identifier(i), i);

    expect_blank_prefixed!(iter, TokenKind::OpeningParenthesis, ());
    let arguments = delemited_by(
        iter,
        TokenKind::Comma,
        &[TokenKind::ClosingParenthesis],
        parse_expr,
    );
    expect_blank_prefixed!(iter, TokenKind::ClosingParenthesis, ());

    ASTStatement::Annotation(ASTAnnotation {
        identifier,
        arguments,
    })
}

pub fn parse_signal<'a>(iter: &mut Peekable<impl Iterator<Item = Token<'a>>>) -> ASTSignal<'a> {
    expect_blank_prefixed!(iter, TokenKind::Signal, ());

    let identifier = expect_blank_prefixed!(iter, TokenKind::Identifier(s), s);

    expect_blank_prefixed!(iter, TokenKind::OpeningParenthesis, ());
    let parameters = delemited_by(
        iter,
        TokenKind::Comma,
        &[TokenKind::ClosingParenthesis],
        |iter| parse_variable_body(iter, ASTVariableKind::FunctionParameter),
    );
    expect_blank_prefixed!(iter, TokenKind::ClosingParenthesis, ());

    ASTSignal {
        identifier,
        parameters,
    }
}
