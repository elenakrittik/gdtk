use std::iter::Peekable;

use gdtk_ast::poor::{ASTAnnotation, ASTSignal, ASTVariableKind};
use gdtk_lexer::{Token, TokenKind};

use crate::{
    expressions::parse_expr,
    utils::{delemited_by, expect},
    variables::parse_variable_body,
};

pub fn parse_annotation<'a>(
    iter: &mut Peekable<impl Iterator<Item = Token<'a>>>,
) -> ASTAnnotation<'a> {
    expect!(iter, TokenKind::Annotation);

    let identifier = expect!(iter, TokenKind::Identifier(i), i);

    expect!(iter, TokenKind::OpeningParenthesis);
    let arguments = delemited_by(
        iter,
        TokenKind::Comma,
        &[TokenKind::ClosingParenthesis],
        parse_expr,
    );
    expect!(iter, TokenKind::ClosingParenthesis);

    ASTAnnotation {
        identifier,
        arguments,
    }
}

pub fn parse_signal<'a>(iter: &mut Peekable<impl Iterator<Item = Token<'a>>>) -> ASTSignal<'a> {
    expect!(iter, TokenKind::Signal);

    let identifier = expect!(iter, TokenKind::Identifier(s), s);

    expect!(iter, TokenKind::OpeningParenthesis);
    let parameters = delemited_by(
        iter,
        TokenKind::Comma,
        &[TokenKind::ClosingParenthesis],
        |iter| parse_variable_body(iter, ASTVariableKind::Binding),
    );
    expect!(iter, TokenKind::ClosingParenthesis);

    ASTSignal {
        identifier,
        parameters,
    }
}
