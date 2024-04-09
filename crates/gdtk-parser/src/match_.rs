use std::iter::Peekable;

use gdtk_ast::poor::{
    ASTMatchArm, ASTMatchPattern, ASTMatchStmt, ASTVariable, DictPattern,
};
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

    ASTMatchArm {
        pattern,
        guard,
        block,
    }
}

/// Parse a match arm pattern, including alternatives.
pub fn parse_match_pattern<'a>(
    iter: &mut Peekable<impl Iterator<Item = Token<'a>>>,
) -> ASTMatchPattern<'a> {
    let pats = delemited_by(
        iter,
        TokenKind::Comma,
        &[TokenKind::When, TokenKind::Colon],
        parse_raw_match_pattern,
    );

    if pats.len() == 1 {
        pats.into_iter().next().unwrap()
    } else {
        ASTMatchPattern::Alternative(pats)
    }
}

/// Parse a match arm pattern without checking for alternatives.
fn parse_raw_match_pattern<'a>(
    iter: &mut Peekable<impl Iterator<Item = Token<'a>>>,
) -> ASTMatchPattern<'a> {
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

fn parse_match_binding_pattern<'a>(
    iter: &mut Peekable<impl Iterator<Item = Token<'a>>>,
) -> ASTMatchPattern<'a> {
    expect!(iter, TokenKind::Var);

    let identifier = expect!(iter, TokenKind::Identifier(s), s);

    ASTMatchPattern::Binding(ASTVariable::new_binding(identifier))
}

fn parse_match_array_pattern<'a>(
    iter: &mut Peekable<impl Iterator<Item = Token<'a>>>,
) -> ASTMatchPattern<'a> {
    expect!(iter, TokenKind::OpeningBracket);

    let patterns = delemited_by(
        iter,
        TokenKind::Comma,
        &[TokenKind::ClosingBracket],
        parse_raw_match_pattern,
    );

    expect!(iter, TokenKind::ClosingBracket);

    ASTMatchPattern::Array(patterns)
}

fn parse_match_dict_pattern<'a>(
    iter: &mut Peekable<impl Iterator<Item = Token<'a>>>,
) -> ASTMatchPattern<'a> {
    expect!(iter, TokenKind::OpeningBrace);

    fn callback<'a>(iter: &mut Peekable<impl Iterator<Item = Token<'a>>>) -> DictPattern<'a> {
        let key = parse_expr(iter);

        let value = if iter.peek().is_some_and(|t| t.kind == TokenKind::Colon) {
            expect!(iter, TokenKind::Colon);

            let value = parse_raw_match_pattern(iter);

            Some(Box::new(value))
        } else {
            None
        };

        (key, value)
    }

    let pairs = delemited_by(iter, TokenKind::Comma, &[TokenKind::ClosingBrace], callback);

    expect!(iter, TokenKind::ClosingBrace);

    ASTMatchPattern::Dictionary(pairs)
}

#[cfg(test)]
mod tests {
    use gdtk_ast::poor::*;

    use crate::match_::{parse_match_arm, parse_match_pattern};
    use crate::test_utils::create_parser;

    #[test]
    fn test_value_pattern() {
        let mut parser = create_parser("literal");
        let result = parse_match_pattern(&mut parser);
        let expected = ASTMatchPattern::Value(ASTValue::Identifier("literal"));

        assert_eq!(result, expected);
    }

    #[test]
    fn test_array_pattern() {
        let mut parser = create_parser("[literal]");
        let result = parse_match_pattern(&mut parser);
        let expected = ASTMatchPattern::Array(vec![ASTMatchPattern::Value(ASTValue::Identifier(
            "literal",
        ))]);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_alternative_pattern() {
        let mut parser = create_parser("literal1, literal2");
        let result = parse_match_pattern(&mut parser);
        let expected = ASTMatchPattern::Alternative(vec![
            ASTMatchPattern::Value(ASTValue::Identifier("literal1")),
            ASTMatchPattern::Value(ASTValue::Identifier("literal2")),
        ]);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_match_arm_empty_body() {
        let mut parser = create_parser("literal:\n    pass");
        let result = parse_match_arm(&mut parser);
        let expected = ASTMatchArm {
            pattern: ASTMatchPattern::Value(ASTValue::Identifier("literal")),
            guard: None,
            block: vec![ASTStatement::Pass],
        };

        assert_eq!(result, expected);
    }

    #[test]
    fn test_match_arm_guard() {
        let mut parser = create_parser("literal when expr:\n    pass");
        let result = parse_match_arm(&mut parser);
        let expected = ASTMatchArm {
            pattern: ASTMatchPattern::Value(ASTValue::Identifier("literal")),
            guard: Some(ASTValue::Identifier("expr")),
            block: vec![ASTStatement::Pass],
        };

        assert_eq!(result, expected);
    }

    #[test]
    fn test_match_arm_block() {
        let mut parser = create_parser("literal:\n    1\n    2\n    3");
        let result = parse_match_arm(&mut parser);
        let expected = ASTMatchArm {
            pattern: ASTMatchPattern::Value(ASTValue::Identifier("literal")),
            guard: None,
            block: vec![
                ASTStatement::Value(ASTValue::Number(1)),
                ASTStatement::Value(ASTValue::Number(2)),
                ASTStatement::Value(ASTValue::Number(3)),
            ],
        };

        assert_eq!(result, expected);
    }

    #[test]
    fn test_match_arm_guard_and_body() {
        let mut parser = create_parser("literal when expr:\n    1\n    2\n    3");
        let result = parse_match_arm(&mut parser);
        let expected = ASTMatchArm {
            pattern: ASTMatchPattern::Value(ASTValue::Identifier("literal")),
            guard: Some(ASTValue::Identifier("expr")),
            block: vec![
                ASTStatement::Value(ASTValue::Number(1)),
                ASTStatement::Value(ASTValue::Number(2)),
                ASTStatement::Value(ASTValue::Number(3)),
            ],
        };

        assert_eq!(result, expected);
    }
}
