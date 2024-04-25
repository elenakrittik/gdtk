use gdtk_ast::{ASTClassStmt, ASTEnumStmt, ASTEnumVariant};
use gdtk_lexer::{Token, TokenKind};

use crate::block::parse_block;
use crate::expressions::parse_expr;
use crate::utils::{advance_and_parse, delemited_by, expect, parse_ident};
use crate::Parser;

pub fn parse_enum<'a>(parser: &mut Parser<'a, impl Iterator<Item = Token<'a>>>) -> ASTEnumStmt<'a> {
    let start = parser.span_start();

    expect!(parser, TokenKind::Enum);

    let identifier = if parser
        .peek()
        .is_some_and(|t| matches!(t.kind, TokenKind::Identifier(_)))
    {
        Some(parse_ident(parser))
    } else {
        None
    };

    expect!(parser, TokenKind::OpeningBrace);

    fn parse_enum_variant<'a>(
        parser: &mut Parser<'a, impl Iterator<Item = Token<'a>>>,
    ) -> ASTEnumVariant<'a> {
        let start = parser.span_start();

        let identifier = parse_ident(parser);

        let value = if parser.peek().is_some_and(|t| t.kind.is_assignment()) {
            Some(advance_and_parse(parser, parse_expr))
        } else {
            None
        };

        ASTEnumVariant {
            identifier,
            value,
            span: parser.finish_span(start),
        }
    }

    let variants = parser.with_parens_ctx(true, |parser| {
        delemited_by(
            parser,
            TokenKind::Comma,
            &[TokenKind::ClosingBrace],
            parse_enum_variant,
        )
    });

    expect!(parser, TokenKind::ClosingBrace);

    ASTEnumStmt {
        identifier,
        variants,
        span: parser.finish_span(start),
    }
}

pub fn parse_class<'a>(
    parser: &mut Parser<'a, impl Iterator<Item = Token<'a>>>,
) -> ASTClassStmt<'a> {
    expect!(parser, TokenKind::Class);

    let identifier = parse_ident(parser);
    let mut extends = None;

    if parser
        .peek()
        .is_some_and(|t| matches!(t.kind, TokenKind::Extends))
    {
        parser.next();
        extends = Some(parse_ident(parser));
    }

    expect!(parser, TokenKind::Colon);

    let body = parse_block(parser, false);

    ASTClassStmt {
        identifier,
        extends,
        body,
    }
}

#[cfg(test)]
mod tests {
    use gdtk_ast::*;

    use crate::classes::{parse_class, parse_enum};
    use crate::test_utils::{create_parser, make_ident, make_number, make_string};

    const PASS_STMT: ASTStatement = ASTStatement::Pass(ASTPassStmt { span: 0..0 });

    #[test]
    fn test_parse_class() {
        let mut parser = create_parser("class MyClass:\n    pass");
        let expected = ASTClassStmt {
            identifier: make_ident("MyClass"),
            extends: None,
            body: vec![PASS_STMT],
        };
        let result = parse_class(&mut parser);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_class_extends() {
        let mut parser = create_parser("class MyClass extends AnotherClass:\n    pass");
        let expected = ASTClassStmt {
            identifier: make_ident("MyClass"),
            extends: Some(make_ident("AnotherClass")),
            body: vec![PASS_STMT],
        };
        let result = parse_class(&mut parser);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_enum_empty_unnamed() {
        let mut parser = create_parser("enum {}");
        let expected = ASTEnumStmt {
            identifier: None,
            variants: vec![],
            span: 0..0,
        };
        let result = parse_enum(&mut parser);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_enum_empty_named() {
        let mut parser = create_parser("enum State {}");
        let expected = ASTEnumStmt {
            identifier: Some(make_ident("State")),
            variants: vec![],
            span: 0..0,
        };
        let result = parse_enum(&mut parser);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_enum_normal_unnamed() {
        let mut parser = create_parser("enum { WALKING, JUMPING }");
        let expected = ASTEnumStmt {
            identifier: None,
            variants: vec![
                ASTEnumVariant {
                    identifier: make_ident("WALKING"),
                    value: None,
                    span: 0..0,
                },
                ASTEnumVariant {
                    identifier: make_ident("JUMPING"),
                    value: None,
                    span: 0..0,
                },
            ],
            span: 0..0,
        };
        let result = parse_enum(&mut parser);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_enum_normal_named() {
        let mut parser = create_parser("enum State { WALKING, JUMPING }");
        let expected = ASTEnumStmt {
            identifier: Some(make_ident("State")),
            variants: vec![
                ASTEnumVariant {
                    identifier: make_ident("WALKING"),
                    value: None,
                    span: 0..0,
                },
                ASTEnumVariant {
                    identifier: make_ident("JUMPING"),
                    value: None,
                    span: 0..0,
                },
            ],
            span: 0..0,
        };
        let result = parse_enum(&mut parser);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_enum_with_values_unnamed() {
        let mut parser = create_parser("enum { WALKING = 1, JUMPING = 'invalid' }");
        let expected = ASTEnumStmt {
            identifier: None,
            variants: vec![
                ASTEnumVariant {
                    identifier: make_ident("WALKING"),
                    value: Some(make_number(1)),
                    span: 0..0,
                },
                ASTEnumVariant {
                    identifier: make_ident("JUMPING"),
                    value: Some(make_string("invalid")),
                    span: 0..0,
                },
            ],
            span: 0..0,
        };
        let result = parse_enum(&mut parser);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_enum_with_values_named() {
        let mut parser = create_parser("enum State { WALKING = 1, JUMPING = 'invalid' }");
        let expected = ASTEnumStmt {
            identifier: Some(make_ident("State")),
            variants: vec![
                ASTEnumVariant {
                    identifier: make_ident("WALKING"),
                    value: Some(make_number(1)),
                    span: 0..0,
                },
                ASTEnumVariant {
                    identifier: make_ident("JUMPING"),
                    value: Some(make_string("invalid")),
                    span: 0..0,
                },
            ],
            span: 0..0,
        };
        let result = parse_enum(&mut parser);

        assert_eq!(result, expected);
    }
}
