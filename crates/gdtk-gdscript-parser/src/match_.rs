use gdtk_ast::{ASTMatchArm, ASTMatchPattern, ASTMatchStmt, ASTVariable, DictPattern};
use crate::lexer::{Token, TokenKind};

use crate::block::parse_block;
use crate::expressions::parse_expr;
use crate::utils::{advance_and_parse, delemited_by, expect, parse_ident};
use crate::Parser;

/// Parse a match statement.
pub fn parse_match<'a>(
    parser: &mut Parser<'a, impl Iterator<Item = Token<'a>>>,
) -> ASTMatchStmt<'a> {
    expect!(parser, TokenKind::Match);

    let expr = parse_expr(parser);

    expect!(parser, TokenKind::Colon);
    expect!(parser, TokenKind::Newline);
    expect!(parser, TokenKind::Indent);

    let mut arms = vec![];

    while parser.peek().is_some_and(|t| !t.kind.is_dedent()) {
        arms.push(parse_match_arm(parser));
    }

    parser.next(); // guaranteed to be a TokenKind::Dedent already

    ASTMatchStmt { expr, arms }
}

/// Parse a match arm.
pub fn parse_match_arm<'a>(
    parser: &mut Parser<'a, impl Iterator<Item = Token<'a>>>,
) -> ASTMatchArm<'a> {
    let pattern = parse_match_pattern(parser);

    let guard = if parser.peek().is_some_and(|t| t.kind.is_when()) {
        Some(advance_and_parse(parser, parse_expr))
    } else {
        None
    };

    expect!(parser, TokenKind::Colon);

    let block = parse_block(parser, false);

    ASTMatchArm {
        pattern,
        guard,
        block,
    }
}

/// Parse a match arm pattern, including alternatives.
pub fn parse_match_pattern<'a>(
    parser: &mut Parser<'a, impl Iterator<Item = Token<'a>>>,
) -> ASTMatchPattern<'a> {
    let pats = delemited_by(
        parser,
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
    parser: &mut Parser<'a, impl Iterator<Item = Token<'a>>>,
) -> ASTMatchPattern<'a> {
    match parser
        .peek()
        .expect("unexpected EOF, expected a pattern")
        .kind
    {
        TokenKind::Range => advance_and_parse(parser, |_| ASTMatchPattern::Ignore),
        TokenKind::Var => parse_match_binding_pattern(parser),
        TokenKind::OpeningBracket => parse_match_array_pattern(parser),
        TokenKind::OpeningBrace => parse_match_dict_pattern(parser),
        _ => ASTMatchPattern::Value(parse_expr(parser)),
    }
}

fn parse_match_binding_pattern<'a>(
    parser: &mut Parser<'a, impl Iterator<Item = Token<'a>>>,
) -> ASTMatchPattern<'a> {
    // Apparently, even in multiline patterns, binding subpatterns do not allow newlines between
    // "var" and the identifier.
    let identifier = parser.with_parens_ctx(false, |parser| {
        expect!(parser, TokenKind::Var);

        parse_ident(parser)
    });

    ASTMatchPattern::Binding(ASTVariable::new_binding(identifier))
}

fn parse_match_array_pattern<'a>(
    parser: &mut Parser<'a, impl Iterator<Item = Token<'a>>>,
) -> ASTMatchPattern<'a> {
    expect!(parser, TokenKind::OpeningBracket);

    let patterns = parser.with_parens_ctx(true, |parser| {
        delemited_by(
            parser,
            TokenKind::Comma,
            &[TokenKind::ClosingBracket],
            parse_raw_match_pattern,
        )
    });

    expect!(parser, TokenKind::ClosingBracket);

    ASTMatchPattern::Array(patterns)
}

fn parse_match_dict_pattern<'a>(
    parser: &mut Parser<'a, impl Iterator<Item = Token<'a>>>,
) -> ASTMatchPattern<'a> {
    expect!(parser, TokenKind::OpeningBrace);

    fn callback<'a>(parser: &mut Parser<'a, impl Iterator<Item = Token<'a>>>) -> DictPattern<'a> {
        let key = parse_expr(parser);

        let value = if parser.peek().is_some_and(|t| t.kind == TokenKind::Colon) {
            expect!(parser, TokenKind::Colon);

            let value = parse_raw_match_pattern(parser);

            Some(Box::new(value))
        } else {
            None
        };

        (key, value)
    }

    let pairs = parser.with_parens_ctx(true, |parser| {
        delemited_by(
            parser,
            TokenKind::Comma,
            &[TokenKind::ClosingBrace],
            callback,
        )
    });

    expect!(parser, TokenKind::ClosingBrace);

    ASTMatchPattern::Dictionary(pairs)
}

#[cfg(test)]
mod tests {
    use gdtk_ast::*;

    use crate::match_::{parse_match, parse_match_arm, parse_match_pattern};
    use crate::test_utils::{create_parser, make_ident, make_number, PASS_STMT};

    #[test]
    fn test_value_pattern() {
        let mut parser = create_parser("literal");
        let result = parse_match_pattern(&mut parser);
        let expected = ASTMatchPattern::Value(make_ident("literal"));

        assert_eq!(result, expected);
    }

    #[test]
    fn test_binding_pattern() {
        let mut parser = create_parser("var x");
        let result = parse_match_pattern(&mut parser);
        let expected = ASTMatchPattern::Binding(ASTVariable::new_binding(make_ident("x")));

        assert_eq!(result, expected);
    }

    #[test]
    fn test_array_pattern() {
        let mut parser = create_parser("[literal]");
        let result = parse_match_pattern(&mut parser);
        let expected = ASTMatchPattern::Array(vec![ASTMatchPattern::Value(make_ident("literal"))]);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_dictionary_pattern() {
        let mut parser = create_parser("{ x: var y, z }");
        let result = parse_match_pattern(&mut parser);
        let expected = ASTMatchPattern::Dictionary(vec![
            (
                make_ident("x"),
                Some(Box::new(ASTMatchPattern::Binding(
                    ASTVariable::new_binding(make_ident("y")),
                ))),
            ),
            (make_ident("z"), None),
        ]);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_alternative_pattern() {
        let mut parser = create_parser("literal1, literal2");
        let result = parse_match_pattern(&mut parser);
        let expected = ASTMatchPattern::Alternative(vec![
            ASTMatchPattern::Value(make_ident("literal1")),
            ASTMatchPattern::Value(make_ident("literal2")),
        ]);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_match_arm_empty_body() {
        let mut parser = create_parser("literal:\n    pass");
        let result = parse_match_arm(&mut parser);
        let expected = ASTMatchArm {
            pattern: ASTMatchPattern::Value(make_ident("literal")),
            guard: None,
            block: vec![PASS_STMT],
        };

        assert_eq!(result, expected);
    }

    #[test]
    fn test_match_arm_guard() {
        let mut parser = create_parser("literal when expr:\n    pass");
        let result = parse_match_arm(&mut parser);
        let expected = ASTMatchArm {
            pattern: ASTMatchPattern::Value(make_ident("literal")),
            guard: Some(make_ident("expr")),
            block: vec![PASS_STMT],
        };

        assert_eq!(result, expected);
    }

    #[test]
    fn test_match_arm_block() {
        let mut parser = create_parser("literal:\n    1\n    2\n    3");
        let result = parse_match_arm(&mut parser);
        let expected = ASTMatchArm {
            pattern: ASTMatchPattern::Value(make_ident("literal")),
            guard: None,
            block: vec![
                ASTStatement::Expr(make_number(1)),
                ASTStatement::Expr(make_number(2)),
                ASTStatement::Expr(make_number(3)),
            ],
        };

        assert_eq!(result, expected);
    }

    #[test]
    fn test_match_arm_guard_and_body() {
        let mut parser = create_parser("literal when expr:\n    1\n    2\n    3");
        let result = parse_match_arm(&mut parser);
        let expected = ASTMatchArm {
            pattern: ASTMatchPattern::Value(make_ident("literal")),
            guard: Some(make_ident("expr")),
            block: vec![
                ASTStatement::Expr(make_number(1)),
                ASTStatement::Expr(make_number(2)),
                ASTStatement::Expr(make_number(3)),
            ],
        };

        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_match() {
        let mut parser = create_parser("match expr:\n    _: pass");
        let result = parse_match(&mut parser);
        let expected = ASTMatchStmt {
            expr: make_ident("expr"),
            arms: vec![ASTMatchArm {
                pattern: ASTMatchPattern::Value(make_ident("_")),
                guard: None,
                block: vec![PASS_STMT],
            }],
        };

        assert_eq!(result, expected);
    }

    #[test]
    fn test_multiline_dict_and_array_patterns() {
        let mut parser = create_parser("{ x:\n var y }, [\nz\n]");
        let result = parse_match_pattern(&mut parser);
        let expected = ASTMatchPattern::Alternative(vec![
            ASTMatchPattern::Dictionary(vec![(
                make_ident("x"),
                Some(Box::new(ASTMatchPattern::Binding(
                    ASTVariable::new_binding(make_ident("y")),
                ))),
            )]),
            ASTMatchPattern::Array(vec![ASTMatchPattern::Value(make_ident("z"))]),
        ]);

        assert_eq!(result, expected);
    }
}
