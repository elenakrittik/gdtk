use std::iter::Peekable;

use gdtk_ast::poor::{ASTFunction, ASTStatement};
use gdtk_lexer::{Token, TokenKind};

use crate::block::parse_block;
use crate::utils::{collect_params, expect_blank_prefixed, peek_non_blank};
use crate::values::parse_value;

pub fn parse_func<'a, T>(iter: &mut Peekable<T>) -> ASTStatement<'a>
where
    T: Iterator<Item = Token<'a>>,
{
    let identifier = expect_blank_prefixed!(iter, TokenKind::Identifier(s), s);
    let mut return_type = None;

    let parameters = collect_params(iter);

    if matches!(peek_non_blank!(iter).kind, TokenKind::Arrow) {
        iter.next();
        return_type = Some(parse_value(iter, None));
    }

    expect_blank_prefixed!(iter, TokenKind::Colon, ());

    let body = parse_block(iter);

    ASTStatement::Func(ASTFunction {
        identifier,
        parameters,
        return_type: return_type.map(Box::new),
        body,
    })
}
