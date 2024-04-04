use std::iter::Peekable;

use gdtk_ast::poor::{ASTClass, ASTEnum, ASTEnumVariant};
use gdtk_lexer::{Token, TokenKind};

use crate::block::parse_block;
use crate::expressions::parse_expr;
use crate::utils::{advance_and_parse, delemited_by, expect, peek_non_blank};

pub fn parse_enum<'a>(iter: &mut Peekable<impl Iterator<Item = Token<'a>>>) -> ASTEnum<'a> {
    expect!(iter, TokenKind::Enum);

    let identifier =
        if peek_non_blank(iter).is_some_and(|t| matches!(t.kind, TokenKind::Identifier(_))) {
            Some(iter.next().unwrap().kind.into_identifier().unwrap())
        } else {
            None
        };

    expect!(iter, TokenKind::OpeningBrace);

    fn parse_enum_variant<'a>(
        iter: &mut Peekable<impl Iterator<Item = Token<'a>>>,
    ) -> ASTEnumVariant<'a> {
        let identifier = expect!(iter, TokenKind::Identifier(s), s);

        let value = if peek_non_blank(iter).is_some_and(|t| t.kind.is_assignment()) {
            Some(advance_and_parse(iter, parse_expr))
        } else {
            None
        };

        ASTEnumVariant { identifier, value }
    }

    let variants = delemited_by(
        iter,
        TokenKind::Comma,
        &[TokenKind::ClosingBrace],
        parse_enum_variant,
    );

    expect!(iter, TokenKind::ClosingBrace);

    ASTEnum {
        identifier,
        variants,
    }
}

pub fn parse_class<'a>(iter: &mut Peekable<impl Iterator<Item = Token<'a>>>) -> ASTClass<'a> {
    expect!(iter, TokenKind::Class);
    let identifier = expect!(iter, TokenKind::Identifier(s), s);
    let mut extends = None;

    if peek_non_blank(iter).is_some_and(|t| matches!(t.kind, TokenKind::Extends)) {
        iter.next();
        extends = Some(expect!(iter, TokenKind::Identifier(s), s));
    }

    expect!(iter, TokenKind::Colon);

    let body = parse_block(iter, false);

    ASTClass {
        identifier,
        extends,
        body,
    }
}
