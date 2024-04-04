use std::iter::Peekable;

use gdtk_ast::poor::{ASTFunction, ASTVariableKind};
use gdtk_lexer::{Token, TokenKind};

use crate::block::parse_block;
use crate::expressions::parse_expr;
use crate::statement::parse_statement;
use crate::utils::{delemited_by, expect_blank_prefixed, peek_non_blank};
use crate::variables::parse_variable_body;

pub fn parse_func<'a>(
    iter: &mut Peekable<impl Iterator<Item = Token<'a>>>,
    lambda: bool,
) -> ASTFunction<'a> {
    expect_blank_prefixed!(iter, TokenKind::Func, ());

    let mut identifier = None;
    let mut return_type = None;
    #[allow(unused_assignments)] // false positive
    let mut body = None;

    if peek_non_blank(iter).is_some_and(|t| matches!(t.kind, TokenKind::Identifier(_))) {
        identifier = Some(expect_blank_prefixed!(iter, TokenKind::Identifier(s), s));
    }

    expect_blank_prefixed!(iter, TokenKind::OpeningParenthesis, ());

    let parameters = delemited_by(
        iter,
        TokenKind::Comma,
        &[TokenKind::ClosingParenthesis],
        |iter| parse_variable_body(iter, ASTVariableKind::Binding),
    );

    expect_blank_prefixed!(iter, TokenKind::ClosingParenthesis, ());

    if peek_non_blank(iter).is_some_and(|t| matches!(t.kind, TokenKind::Arrow)) {
        iter.next();
        return_type = Some(parse_expr(iter));
    }

    expect_blank_prefixed!(iter, TokenKind::Colon, ());

    if peek_non_blank(iter).is_some_and(|t| matches!(t.kind, TokenKind::Newline)) {
        body = Some(parse_block(iter, lambda));
    } else {
        body = Some(vec![parse_statement(iter)]);
    }

    ASTFunction {
        identifier,
        parameters,
        return_type: return_type.map(Box::new),
        body: body.unwrap(),
    }
}
