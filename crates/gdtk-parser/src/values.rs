use std::iter::Peekable;

use gdtk_ast::poor::{ASTValue, DictValue};
use gdtk_lexer::{Token, TokenKind};

use crate::{expressions::parse_expr, utils::{expect_blank_prefixed, next_non_blank, peek_non_blank}};

pub fn parse_dictionary<'a>(iter: &mut Peekable<impl Iterator<Item = Token<'a>>>) -> DictValue<'a> {
    let mut vec: DictValue<'a> = vec![];

    expect_blank_prefixed!(iter, TokenKind::OpeningBrace, ());

    match peek_non_blank(iter).expect("unexpected EOF").kind {
        TokenKind::ClosingBrace => { iter.next(); }, // empty dict
        TokenKind::Identifier(_) => {
            let first_key = iter.next().unwrap().kind.into_identifier().unwrap();
            parse_lua_dict(iter, &mut vec, ASTValue::String(first_key));
        },
        _ => {
            eprintln!("parsing dict, looking for pythonish key");
            eprintln!("next tkn: {:?}", peek_non_blank(iter));
            let first_key = parse_expr(iter);
            parse_python_dict(iter, &mut vec, first_key);
        },
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
    let first_val = parse_expr(iter);
    vec.push((first_key, first_val));

    let mut expect_comma = true; // just got our pair, expect a comma

    loop {
        match next_non_blank(iter) {
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
                vec.push((ASTValue::String(s), parse_expr(iter)));
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
    let first_val = parse_expr(iter);
    vec.push((first_key, first_val));

    let mut expect_comma = true; // just got our pair, expect a comma

    loop {
        match peek_non_blank(iter).expect("unexpected EOF").kind {
            TokenKind::Comma => {
                iter.next();

                if !expect_comma {
                    panic!("unexpected comma, expected a value");
                }
                expect_comma = false;
            }
            TokenKind::ClosingBrace => {
                iter.next();
                break;
            },
            _ => {
                let key = parse_expr(iter);
                expect_blank_prefixed!(iter, TokenKind::Colon, ());
                vec.push((key, parse_expr(iter)));
                expect_comma = true;
            }
        }
    }
}
