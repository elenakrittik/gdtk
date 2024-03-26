use std::iter::Peekable;

use gdtk_ast::poor::ASTFunction;
use gdtk_lexer::{Token, TokenKind};

use crate::block::parse_block;
use crate::statement::parse_statement;
use crate::utils::{collect_params, expect_blank_prefixed, peek_non_blank};
use crate::values::parse_value;

pub fn parse_func<'a, T>(iter: &mut Peekable<T>) -> ASTFunction<'a>
where
    T: Iterator<Item = Token<'a>>,
{
    let mut identifier = None;
    let mut return_type = None;
    let mut body = None;

    if matches!(peek_non_blank!(iter).kind, TokenKind::Identifier(_)) {
        identifier = Some(expect_blank_prefixed!(iter, TokenKind::Identifier(s), s));
    }

    let parameters = collect_params(iter);

    if matches!(peek_non_blank!(iter).kind, TokenKind::Arrow) {
        iter.next();
        return_type = Some(parse_value(iter, None));
    }

    expect_blank_prefixed!(iter, TokenKind::Colon, ());

    if matches!(peek_non_blank!(iter).kind, TokenKind::Newline) {
        body = Some(parse_block(iter));
    } else {
        body = Some(vec![parse_statement(iter, None)]);
    }

    ASTFunction {
        identifier,
        parameters,
        return_type: return_type.map(Box::new),
        body: body.unwrap(),
    }
}
