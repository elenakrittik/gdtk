use std::iter::Peekable;

use gdtk_ast::poor::{ASTMatchArm, ASTMatchPattern, ASTMatchStmt, ASTVariable, ASTVariableKind};
use gdtk_lexer::{Token, TokenKind};

use crate::block::parse_block;
use crate::expressions::parse_expr;
use crate::utils::{advance_and_parse, delemited_by, expect};

/// Parse a match statement.
pub fn parse_match<'a>(iter: &mut Peekable<impl Iterator<Item = Token<'a>>>) -> ASTMatchStmt<'a> {
    expect!(iter, TokenKind::Match);

    let expr = parse_expr(iter);

    expect!(iter, TokenKind::Colon);
    expect!(iter, TokenKind::Newline);
    expect!(iter, TokenKind::Indent);

    let mut arms = vec![];

    while iter.peek().is_some_and(|t| !t.kind.is_dedent()) {
        arms.push(parse_match_arm(iter));
    }

    iter.next(); // guaranteed to be a TokenKind::Dedent already

    ASTMatchStmt { expr, arms }
}

/// Parse a match arm.
pub fn parse_match_arm<'a>(
    iter: &mut Peekable<impl Iterator<Item = Token<'a>>>,
) -> ASTMatchArm<'a> {
    let pattern = parse_match_pattern(iter);

    let guard = if iter.peek().is_some_and(|t| t.kind.is_when()) {
        Some(advance_and_parse(iter, parse_expr))
    } else {
        None
    };

    expect!(iter, TokenKind::Colon);

    let block = parse_block(iter, false);

    ASTMatchArm { pattern, guard, block }
}

/// Parse a match arm pattern.
pub fn parse_match_pattern<'a>(
    iter: &mut Peekable<impl Iterator<Item = Token<'a>>>,
) -> ASTMatchPattern<'a> {
    fn callback<'a>(iter: &mut Peekable<impl Iterator<Item = Token<'a>>>) -> ASTMatchPattern<'a> {
        match iter
            .peek()
            .expect("unexpected EOF, expected a pattern")
            .kind
        {
            TokenKind::Range => advance_and_parse(iter, |_| ASTMatchPattern::Ignore),
            TokenKind::Var => parse_match_binding_pattern(iter),
            TokenKind::OpeningBracket => parse_match_array_pattern(iter),
            TokenKind::OpeningBrace => parse_match_dict_pattern(iter),
            _ => ASTMatchPattern::Value(parse_expr(iter)),
        }
    }

    let pats = delemited_by(
        iter,
        TokenKind::Comma,
        &[
            TokenKind::When,
            TokenKind::Colon,
            TokenKind::ClosingBracket, // HACK/TODO: figure out why this is needed
        ],
        callback,
    );

    if pats.len() == 1 {
        pats.into_iter().next().unwrap()
    } else {
        ASTMatchPattern::Alternative(pats)
    }
}

fn parse_match_binding_pattern<'a>(
    iter: &mut Peekable<impl Iterator<Item = Token<'a>>>,
) -> ASTMatchPattern<'a> {
    iter.next();

    let identifier = expect!(iter, TokenKind::Identifier(s), s);

    ASTMatchPattern::Binding(ASTVariable {
        identifier,
        infer_type: true,
        typehint: None,
        value: None,
        kind: ASTVariableKind::Binding,
    })
}

fn parse_match_array_pattern<'a>(
    iter: &mut Peekable<impl Iterator<Item = Token<'a>>>,
) -> ASTMatchPattern<'a> {
    expect!(iter, TokenKind::OpeningBracket);

    let patterns = delemited_by(
        iter,
        TokenKind::Comma,
        &[TokenKind::ClosingBracket],
        parse_match_pattern,
    );

    expect!(iter, TokenKind::ClosingBracket);

    ASTMatchPattern::Array(patterns)
}

fn parse_match_dict_pattern<'a>(
    _iter: &mut Peekable<impl Iterator<Item = Token<'a>>>,
) -> ASTMatchPattern<'a> {
    todo!()
}
