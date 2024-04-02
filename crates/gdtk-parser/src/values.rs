use std::iter::Peekable;

use gdtk_ast::poor::{ASTValue, DictValue};
use gdtk_lexer::{Token, TokenKind};

use crate::{
    expressions::parse_expr,
    utils::{expect_blank_prefixed, delemited_by, peek_non_blank},
};

pub fn parse_dictionary<'a>(iter: &mut Peekable<impl Iterator<Item = Token<'a>>>) -> DictValue<'a> {
    expect_blank_prefixed!(iter, TokenKind::OpeningBrace, ());

    let value = match peek_non_blank(iter).expect("unexpected EOF").kind {
        #[rustfmt::skip]
        TokenKind::ClosingBrace => vec![], // empty dict
        TokenKind::Identifier(_) => parse_lua_dict(iter),
        _ => parse_python_dict(iter),
    };

    expect_blank_prefixed!(iter, TokenKind::ClosingBrace, ());

    value
}

/// Parse a lua-style dictionary body.
fn parse_lua_dict<'a>(iter: &mut Peekable<impl Iterator<Item = Token<'a>>>) -> DictValue<'a> {
    fn parse_lua_key_value<'a>(iter: &mut Peekable<impl Iterator<Item = Token<'a>>>) -> (ASTValue<'a>, ASTValue<'a>) {
        let key = ASTValue::Identifier(expect_blank_prefixed!(iter, TokenKind::Identifier(s), s));
        expect_blank_prefixed!(iter, TokenKind::Assignment, ());
        let value = parse_expr(iter);

        (key, value)
    }

    delemited_by(
        iter,
        TokenKind::Comma,
        &[TokenKind::ClosingBrace],
        parse_lua_key_value,
    )
}

/// Parse a python-style dictionary body.
fn parse_python_dict<'a>(iter: &mut Peekable<impl Iterator<Item = Token<'a>>>) -> DictValue<'a> {
    fn parse_python_key_value<'a>(iter: &mut Peekable<impl Iterator<Item = Token<'a>>>) -> (ASTValue<'a>, ASTValue<'a>) {
        let key = parse_expr(iter);
        expect_blank_prefixed!(iter, TokenKind::Colon, ());
        let value = parse_expr(iter);

        (key, value)
    }

    delemited_by(
        iter,
        TokenKind::Comma,
        &[TokenKind::ClosingBrace],
        parse_python_key_value,
    )
}
