use gdtk_ast::{ASTClass, ASTEnum, ASTEnumVariant, ASTExpr};
use gdtk_lexer::{Token, TokenKind};

use crate::block::parse_block;
use crate::expressions::parse_expr;
use crate::utils::{advance_and_parse, delemited_by, expect};
use crate::Parser;

pub fn parse_enum<'a>(parser: &mut Parser<impl Iterator<Item = Token<'a>>>) -> ASTEnum<'a> {
    expect!(parser, TokenKind::Enum);

    let identifier = if parser
        .peek()
        .is_some_and(|t| matches!(t.kind, TokenKind::Identifier(_)))
    {
        Some(ASTExpr::Identifier(expect!(parser, TokenKind::Identifier(s), s)))
    } else {
        None
    };

    expect!(parser, TokenKind::OpeningBrace);

    fn parse_enum_variant<'a>(
        parser: &mut Parser<impl Iterator<Item = Token<'a>>>,
    ) -> ASTEnumVariant<'a> {
        let identifier = ASTExpr::Identifier(expect!(parser, TokenKind::Identifier(s), s));

        let value = if parser.peek().is_some_and(|t| t.kind.is_assignment()) {
            Some(advance_and_parse(parser, parse_expr))
        } else {
            None
        };

        ASTEnumVariant { identifier, value }
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

    ASTEnum {
        identifier,
        variants,
    }
}

pub fn parse_class<'a>(parser: &mut Parser<impl Iterator<Item = Token<'a>>>) -> ASTClass<'a> {
    expect!(parser, TokenKind::Class);

    let identifier = ASTExpr::Identifier(expect!(parser, TokenKind::Identifier(s), s));
    let mut extends = None;

    if parser
        .peek()
        .is_some_and(|t| matches!(t.kind, TokenKind::Extends))
    {
        parser.next();
        extends = Some(ASTExpr::Identifier(expect!(parser, TokenKind::Identifier(s), s)));
    }

    expect!(parser, TokenKind::Colon);

    let body = parse_block(parser, false);

    ASTClass {
        identifier,
        extends,
        body,
    }
}

#[cfg(test)]
mod tests {
    use gdtk_ast::*;

    use crate::classes::{parse_class, parse_enum};
    use crate::test_utils::create_parser;

    #[test]
    fn test_parse_class() {
        let mut parser = create_parser("class MyClass:\n    pass");
        let expected = ASTClass {
            identifier: ASTExpr::Identifier("MyClass"),
            extends: None,
            body: vec![ASTStatement::Pass],
        };
        let result = parse_class(&mut parser);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_class_extends() {
        let mut parser = create_parser("class MyClass extends AnotherClass:\n    pass");
        let expected = ASTClass {
            identifier: ASTExpr::Identifier("MyClass"),
            extends: Some(ASTExpr::Identifier("AnotherClass")),
            body: vec![ASTStatement::Pass],
        };
        let result = parse_class(&mut parser);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_enum_empty_unnamed() {
        let mut parser = create_parser("enum {}");
        let expected = ASTEnum {
            identifier: None,
            variants: vec![],
        };
        let result = parse_enum(&mut parser);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_enum_empty_named() {
        let mut parser = create_parser("enum State {}");
        let expected = ASTEnum {
            identifier: Some(ASTExpr::Identifier("State")),
            variants: vec![],
        };
        let result = parse_enum(&mut parser);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_enum_normal_unnamed() {
        let mut parser = create_parser("enum { WALKING, JUMPING }");
        let expected = ASTEnum {
            identifier: None,
            variants: vec![
                ASTEnumVariant {
                    identifier: ASTExpr::Identifier("WALKING"),
                    value: None,
                },
                ASTEnumVariant {
                    identifier: ASTExpr::Identifier("JUMPING"),
                    value: None,
                },
            ],
        };
        let result = parse_enum(&mut parser);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_enum_normal_named() {
        let mut parser = create_parser("enum State { WALKING, JUMPING }");
        let expected = ASTEnum {
            identifier: Some(ASTExpr::Identifier("State")),
            variants: vec![
                ASTEnumVariant {
                    identifier: ASTExpr::Identifier("WALKING"),
                    value: None,
                },
                ASTEnumVariant {
                    identifier: ASTExpr::Identifier("JUMPING"),
                    value: None,
                },
            ],
        };
        let result = parse_enum(&mut parser);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_enum_with_values_unnamed() {
        let mut parser = create_parser("enum { WALKING = 1, JUMPING = 'invalid' }");
        let expected = ASTEnum {
            identifier: None,
            variants: vec![
                ASTEnumVariant {
                    identifier: ASTExpr::Identifier("WALKING"),
                    value: Some(ASTExpr::Number(1)),
                },
                ASTEnumVariant {
                    identifier: ASTExpr::Identifier("JUMPING"),
                    value: Some(ASTExpr::String("invalid")),
                },
            ],
        };
        let result = parse_enum(&mut parser);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_enum_with_values_named() {
        let mut parser = create_parser("enum State { WALKING = 1, JUMPING = 'invalid' }");
        let expected = ASTEnum {
            identifier: Some(ASTExpr::Identifier("State")),
            variants: vec![
                ASTEnumVariant {
                    identifier: ASTExpr::Identifier("WALKING"),
                    value: Some(ASTExpr::Number(1)),
                },
                ASTEnumVariant {
                    identifier: ASTExpr::Identifier("JUMPING"),
                    value: Some(ASTExpr::String("invalid")),
                },
            ],
        };
        let result = parse_enum(&mut parser);

        assert_eq!(result, expected);
    }
}
