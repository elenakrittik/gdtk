use gdtk_ast::{ASTAnnotation, ASTExprKind, ASTPostfixOp, ASTSignal, ASTVariableKind};
use gdtk_lexer::{Token, TokenKind};

use crate::{
    expressions::parse_expr,
    utils::{delemited_by, expect},
    variables::parse_variable_body,
    Parser,
};

pub fn parse_annotation<'a>(
    parser: &mut Parser<impl Iterator<Item = Token<'a>>>,
) -> ASTAnnotation<'a> {
    expect!(parser, TokenKind::Annotation);

    let identifier = ASTExprKind::Identifier(expect!(parser, TokenKind::Identifier(i), i));

    let arguments = if parser
        .peek()
        .is_some_and(|t| t.kind.is_opening_parenthesis())
    {
        parser.next();

        let args = delemited_by(
            parser,
            TokenKind::Comma,
            &[TokenKind::ClosingParenthesis],
            parse_expr,
        );

        expect!(parser, TokenKind::ClosingParenthesis);

        Some(args)
    } else {
        None
    };

    ASTAnnotation {
        identifier,
        arguments,
    }
}

pub fn parse_signal<'a>(parser: &mut Parser<impl Iterator<Item = Token<'a>>>) -> ASTSignal<'a> {
    expect!(parser, TokenKind::Signal);

    let identifier = ASTExprKind::Identifier(expect!(parser, TokenKind::Identifier(s), s));

    let parameters = if parser
        .peek()
        .is_some_and(|t| t.kind.is_opening_parenthesis())
    {
        parser.next();

        let params = delemited_by(
            parser,
            TokenKind::Comma,
            &[TokenKind::ClosingParenthesis],
            |iter| parse_variable_body(iter, ASTVariableKind::Binding),
        );

        expect!(parser, TokenKind::ClosingParenthesis);

        Some(params)
    } else {
        None
    };

    ASTSignal {
        identifier,
        parameters,
    }
}

pub fn parse_type<'a>(parser: &mut Parser<impl Iterator<Item = Token<'a>>>) -> ASTExprKind<'a> {
    let base = expect!(parser, TokenKind::Identifier(s), s);

    if parser.peek().is_some_and(|t| t.kind.is_opening_bracket()) {
        parser.next();

        let type_parameters = delemited_by(
            parser,
            TokenKind::Comma,
            &[TokenKind::ClosingBracket],
            parse_type,
        );

        parser.next();

        ASTExprKind::PostfixExpr(
            Box::new(ASTExprKind::Identifier(base)),
            ASTPostfixOp::Subscript(type_parameters),
        )
    } else {
        ASTExprKind::Identifier(base)
    }
}

#[cfg(test)]
mod tests {
    use gdtk_ast::*;

    use super::*;
    use crate::test_utils::create_parser;

    #[test]
    fn test_annotation_empty() {
        let mut parser = create_parser("@annotation");
        let result = parse_annotation(&mut parser);
        let expected = ASTAnnotation {
            identifier: ASTExprKind::Identifier("annotation"),
            arguments: None,
        };

        assert_eq!(result, expected);
    }

    #[test]
    fn test_annotation_zero_args() {
        let mut parser = create_parser("@annotation()");
        let result = parse_annotation(&mut parser);
        let expected = ASTAnnotation {
            identifier: ASTExprKind::Identifier("annotation"),
            arguments: Some(vec![]),
        };

        assert_eq!(result, expected);
    }

    #[test]
    fn test_annotation_one_arg() {
        let mut parser = create_parser("@annotation(0)");
        let result = parse_annotation(&mut parser);
        let expected = ASTAnnotation {
            identifier: ASTExprKind::Identifier("annotation"),
            arguments: Some(vec![ASTExprKind::Number(0)]),
        };

        assert_eq!(result, expected);
    }

    #[test]
    fn test_annotation_two_args() {
        let mut parser = create_parser("@annotation(0, 1)");
        let result = parse_annotation(&mut parser);
        let expected = ASTAnnotation {
            identifier: ASTExprKind::Identifier("annotation"),
            arguments: Some(vec![ASTExprKind::Number(0), ASTExprKind::Number(1)]),
        };

        assert_eq!(result, expected);
    }

    #[test]
    fn test_annotation_trailing_comma() {
        let mut parser = create_parser("@annotation(0,)");
        let result = parse_annotation(&mut parser);
        let expected = ASTAnnotation {
            identifier: ASTExprKind::Identifier("annotation"),
            arguments: Some(vec![ASTExprKind::Number(0)]),
        };

        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_signal_basic() {
        let mut parser = create_parser("signal done");
        let result = parse_signal(&mut parser);
        let expected = ASTSignal {
            identifier: ASTExprKind::Identifier("done"),
            parameters: None,
        };

        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_signal_with_parameters() {
        let mut parser = create_parser("signal done(a, b: int)");
        let result = parse_signal(&mut parser);
        let expected = ASTSignal {
            identifier: ASTExprKind::Identifier("done"),
            parameters: Some(vec![
                ASTVariable {
                    kind: ASTVariableKind::Binding,
                    identifier: ASTExprKind::Identifier("a"),
                    infer_type: false,
                    typehint: None,
                    value: None,
                },
                ASTVariable {
                    kind: ASTVariableKind::Binding,
                    identifier: ASTExprKind::Identifier("b"),
                    infer_type: false,
                    typehint: Some(ASTExprKind::Identifier("int")),
                    value: None,
                },
            ]),
        };

        assert_eq!(result, expected);
    }
}
