use std::iter::Peekable;

use gdtk_ast::poor::{ASTClass, ASTEnum, ASTEnumVariant, ASTStatement};
use gdtk_lexer::{Token, TokenKind};

use crate::block::parse_block;
use crate::utils::{expect_blank_prefixed, next_non_blank, peek_non_blank};
use crate::expressions::parse_expr;

pub fn parse_classname<'a>(iter: &mut Peekable<impl Iterator<Item = Token<'a>>>) -> ASTStatement<'a> {
    expect_blank_prefixed!(iter, TokenKind::ClassName, ());
    expect_blank_prefixed!(iter, TokenKind::Identifier(i), ASTStatement::ClassName(i))
}

pub fn parse_extends<'a>(iter: &mut Peekable<impl Iterator<Item = Token<'a>>>) -> ASTStatement<'a> {
    expect_blank_prefixed!(iter, TokenKind::Extends, ());
    expect_blank_prefixed!(iter, TokenKind::Identifier(i), ASTStatement::Extends(i))
}

pub fn parse_enum<'a>(iter: &mut Peekable<impl Iterator<Item = Token<'a>>>) -> ASTStatement<'a> {
    expect_blank_prefixed!(iter, TokenKind::Enum, ());
    let identifier = match next_non_blank(iter) {
        Token {
            kind: TokenKind::Identifier(s),
            ..
        } => {
            expect_blank_prefixed!(iter, TokenKind::OpeningBrace, ());
            Some(s)
        }
        Token {
            kind: TokenKind::OpeningBrace,
            ..
        } => None,
        other => panic!("unexpected {other:?}, expected identifier or opening brace"),
    };

    let mut variants = vec![];
    let mut expect_comma = false;

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
                kind: TokenKind::Identifier(identifier),
                ..
            } => {
                if expect_comma {
                    panic!("unexpected identifier, expected comma");
                }

                match next_non_blank(iter) {
                    Token {
                        kind: TokenKind::Comma,
                        ..
                    } => variants.push(ASTEnumVariant {
                        identifier,
                        value: None,
                    }),
                    Token {
                        kind: TokenKind::Assignment,
                        ..
                    } => {
                        let value = Some(parse_expr(iter));
                        variants.push(ASTEnumVariant { identifier, value });
                        expect_comma = true;
                    }
                    Token {
                        kind: TokenKind::ClosingBrace,
                        ..
                    } => {
                        variants.push(ASTEnumVariant {
                            identifier,
                            value: None,
                        });
                        break;
                    }
                    other => {
                        panic!("unxpected {other:?}, expected comma, assignment or closing brace")
                    }
                }
            }
            Token {
                kind: TokenKind::ClosingBrace,
                ..
            } => break,
            other => panic!("unexpected {other:?}"),
        }
    }

    ASTStatement::Enum(ASTEnum {
        identifier,
        variants,
    })
}

pub fn parse_class<'a>(iter: &mut Peekable<impl Iterator<Item = Token<'a>>>) -> ASTClass<'a> {
    expect_blank_prefixed!(iter, TokenKind::Class, ());
    let identifier = expect_blank_prefixed!(iter, TokenKind::Identifier(s), s);
    let mut extends = None;

    if peek_non_blank(iter).is_some_and(|t| matches!(t.kind, TokenKind::Extends))
    {
        iter.next();
        extends = Some(expect_blank_prefixed!(iter, TokenKind::Identifier(s), s));
    }

    expect_blank_prefixed!(iter, TokenKind::Colon, ());

    let body = parse_block(iter, false);

    ASTClass {
        identifier,
        extends,
        body,
    }
}
