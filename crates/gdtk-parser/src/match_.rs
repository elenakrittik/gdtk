use std::iter::Peekable;

use gdtk_ast::poor::{ASTMatchStmt, ASTMatchArm, ASTMatchPattern, ASTVariable, ASTVariableKind};
use gdtk_lexer::{Token, TokenKind};

use crate::block::parse_block;
use crate::expressions::parse_expr;
use crate::utils::{delemited_by, expect_blank_prefixed, advance_and_parse};

pub fn parse_match<'a>(iter: &mut Peekable<impl Iterator<Item = Token<'a>>>) -> ASTMatchStmt<'a> {
    expect_blank_prefixed!(iter, TokenKind::Match, ());

    let expr = parse_expr(iter);

    expect_blank_prefixed!(iter, TokenKind::Colon, ());
    expect_blank_prefixed!(iter, TokenKind::Newline, ());
    expect_blank_prefixed!(iter, TokenKind::Indent, ());

    let mut arms = vec![];

    while iter.peek().is_some_and(|t| !t.kind.is_dedent()) {
        arms.push(parse_match_arm(iter));
    }

    iter.next(); // guaranteed to be a TokenKind::Dedent already

    ASTMatchStmt { expr, arms }
}

pub fn parse_match_arm<'a>(iter: &mut Peekable<impl Iterator<Item = Token<'a>>>) -> ASTMatchArm<'a> {
    let pattern = parse_match_pattern(iter);

    expect_blank_prefixed!(iter, TokenKind::Colon, ());

    let block = parse_block(iter, false);

    ASTMatchArm { pattern, block }
}


pub fn parse_match_pattern<'a>(iter: &mut Peekable<impl Iterator<Item = Token<'a>>>) -> ASTMatchPattern<'a> {
    match iter.peek().expect("unexpected EOF, expected a pattern").kind {
        TokenKind::Range => advance_and_parse(iter, |_| ASTMatchPattern::Ignore),
        TokenKind::Var => parse_match_binding_pattern(iter),
        TokenKind::OpeningBracket => parse_match_array_pattern(iter),
        TokenKind::OpeningBrace => parse_match_dict_pattern(iter),
        _ => ASTMatchPattern::Value(parse_expr(iter)),
    }
}

fn parse_match_binding_pattern<'a>(iter: &mut Peekable<impl Iterator<Item = Token<'a>>>) -> ASTMatchPattern<'a> {
    iter.next();

    let identifier = expect_blank_prefixed!(iter, TokenKind::Identifier(s), s);

    ASTMatchPattern::Binding(ASTVariable {
        identifier,
        infer_type: true,
        typehint: None,
        value: None,
        kind: ASTVariableKind::Binding,
    })
}

fn parse_match_array_pattern<'a>(_iter: &mut Peekable<impl Iterator<Item = Token<'a>>>) -> ASTMatchPattern<'a> {
    iter.next();

    delemited_by
}

fn parse_match_dict_pattern<'a>(_iter: &mut Peekable<impl Iterator<Item = Token<'a>>>) -> ASTMatchPattern<'a> {
    todo!()
}
