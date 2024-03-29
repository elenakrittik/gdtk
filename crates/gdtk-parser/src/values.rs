use std::iter::Peekable;

use gdtk_ast::poor::{ASTValue, DictValue};
use gdtk_lexer::{Token, TokenKind};

use crate::utils::{expect_blank_prefixed, next_non_blank};

pub fn parse_dictionary<'a, T>(iter: &mut Peekable<T>) -> DictValue<'a>
where
    T: Iterator<Item = Token<'a>>,
{
    let mut vec: DictValue<'a> = vec![];

    match next_non_blank!(iter) {
        Token {
            kind: TokenKind::ClosingBrace,
            ..
        } => (), // empty dict
        Token {
            kind: TokenKind::Identifier(s),
            ..
        } => parse_lua_dict(iter, &mut vec, ASTValue::String(s)),
        other => {
            let first_key = parse_value(iter, Some(other));
            parse_python_dict(iter, &mut vec, first_key);
        }
    }

    vec
}

pub fn parse_lua_dict<'a, T>(
    iter: &mut Peekable<T>,
    vec: &mut DictValue<'a>,
    first_key: ASTValue<'a>,
) where
    T: Iterator<Item = Token<'a>>,
{
    expect_blank_prefixed!(iter, TokenKind::Assignment, ());
    let first_val = parse_value(iter, None);
    vec.push((first_key, first_val));

    let mut expect_comma = true; // just got our pair, expect a comma

    loop {
        match next_non_blank!(iter) {
            Token {
                kind: TokenKind::Comma,
                ..
            } => {
                if !expect_comma {
                    panic!("unexpected comma, expected a value");
                }
                expect_comma = false;
            }
            Token {
                kind: TokenKind::Identifier(s),
                ..
            } => {
                expect_blank_prefixed!(iter, TokenKind::Assignment, ());
                vec.push((ASTValue::String(s), parse_value(iter, None)));
                expect_comma = true;
            }
            Token {
                kind: TokenKind::ClosingBrace,
                ..
            } => break,
            other => panic!("unexpected {other:?}"),
        }
    }
}

pub fn parse_python_dict<'a, T>(
    iter: &mut Peekable<T>,
    vec: &mut DictValue<'a>,
    first_key: ASTValue<'a>,
) where
    T: Iterator<Item = Token<'a>>,
{
    expect_blank_prefixed!(iter, TokenKind::Colon, ());
    let first_val = parse_value(iter, None);
    vec.push((first_key, first_val));

    let mut expect_comma = true; // just got our pair, expect a comma

    loop {
        match next_non_blank!(iter) {
            Token {
                kind: TokenKind::Comma,
                ..
            } => {
                if !expect_comma {
                    panic!("unexpected comma, expected a value");
                }
                expect_comma = false;
            }
            Token {
                kind: TokenKind::ClosingBrace,
                ..
            } => break,
            other => {
                let key = parse_value(iter, Some(other));
                expect_blank_prefixed!(iter, TokenKind::Colon, ());
                vec.push((key, parse_value(iter, None)));
                expect_comma = true;
            }
        }
    }
}
