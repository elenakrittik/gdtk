use std::iter::Peekable;

use gdtk_ast::poor::{ASTClass, ASTEnum, ASTEnumVariant};
use gdtk_lexer::{Token, TokenKind};

use crate::block::parse_block;
use crate::expressions::parse_expr;
use crate::utils::{advance_and_parse, delemited_by, expect};

pub fn parse_enum<'a>(iter: &mut Peekable<impl Iterator<Item = Token<'a>>>) -> ASTEnum<'a> {
    expect!(iter, TokenKind::Enum);

    let identifier = if iter
        .peek()
        .is_some_and(|t| matches!(t.kind, TokenKind::Identifier(_)))
    {
        Some(iter.next().unwrap().kind.into_identifier().unwrap())
    } else {
        None
    };

    expect!(iter, TokenKind::OpeningBrace);

    fn parse_enum_variant<'a>(
        iter: &mut Peekable<impl Iterator<Item = Token<'a>>>,
    ) -> ASTEnumVariant<'a> {
        let identifier = expect!(iter, TokenKind::Identifier(s), s);

        let value = if iter.peek().is_some_and(|t| t.kind.is_assignment()) {
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

    if iter
        .peek()
        .is_some_and(|t| matches!(t.kind, TokenKind::Extends))
    {
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

#[cfg(test)]
mod tests {
    use gdtk_ast::poor::*;

    use crate::classes::{parse_class, parse_enum};
    use crate::test_utils::create_parser;

    #[test]
    fn test_parse_class() {
        let mut parser = create_parser("class MyClass:\n    pass");
        let expected = ASTClass {
            identifier: "MyClass",
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
            identifier: "MyClass",
            extends: "AnotherClass",
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
            identifier: Some("State"),
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
                ASTEnumVariant { identifier: "WALKING", value: None },
                ASTEnumVariant { identifier: "JUMPING", value: None },
            ],
        };
        let result = parse_enum(&mut parser);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_enum_normal_named() {
        let mut parser = create_parser("enum State { WALKING, JUMPING }");
        let expected = ASTEnum {
            identifier: Some("State"),
            variants: vec![
                ASTEnumVariant { identifier: "WALKING", value: None },
                ASTEnumVariant { identifier: "JUMPING", value: None },
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
                ASTEnumVariant { identifier: "WALKING", value: Some(ASTValue::Number(1)) },
                ASTEnumVariant { identifier: "JUMPING", value: Some(ASTValue::String("invalid")) },
            ],
        };
        let result = parse_enum(&mut parser);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_enum_with_values_named() {
        let mut parser = create_parser("enum State { WALKING = 1, JUMPING = 'invalid' }");
        let expected = ASTEnum {
            identifier: Some("State"),
            variants: vec![
                ASTEnumVariant { identifier: "WALKING", value: Some(ASTValue::Number(1)) },
                ASTEnumVariant { identifier: "JUMPING", value: Some(ASTValue::String("invalid")) },
            ],
        };
        let result = parse_enum(&mut parser);

        assert_eq!(result, expected);
    }
}
