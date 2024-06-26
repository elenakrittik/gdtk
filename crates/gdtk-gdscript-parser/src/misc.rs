use gdtk_gdscript_ast::{
    ASTAnnotationStmt, ASTExpr, ASTExprKind, ASTPostfixOp, ASTPostfixOpKind, ASTSignalStmt,
    ASTVariableKind,
};

use crate::lexer::{Token, TokenKind};
use crate::{
    expressions::parse_expr,
    utils::{delemited_by, expect, parse_ident},
    variables::parse_variable_body,
    Parser,
};

pub fn parse_annotation<'a>(
    parser: &mut Parser<'a, impl Iterator<Item = Token<'a>>>,
) -> ASTAnnotationStmt<'a> {
    expect!(parser, TokenKind::Annotation);

    let identifier = parse_ident(parser);

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

    ASTAnnotationStmt {
        identifier,
        arguments,
    }
}

pub fn parse_signal<'a>(
    parser: &mut Parser<'a, impl Iterator<Item = Token<'a>>>,
) -> ASTSignalStmt<'a> {
    expect!(parser, TokenKind::Signal);

    let identifier = parse_ident(parser);

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

    ASTSignalStmt {
        identifier,
        parameters,
    }
}

pub fn parse_type<'a>(parser: &mut Parser<'a, impl Iterator<Item = Token<'a>>>) -> ASTExpr<'a> {
    let ident = parse_ident(parser);

    if parser.peek().is_some_and(|t| t.kind.is_opening_bracket()) {
        let start = parser.span_start();

        parser.next();

        let type_parameters = delemited_by(
            parser,
            TokenKind::Comma,
            &[TokenKind::ClosingBracket],
            parse_type,
        );

        parser.next();

        let type_parameters_span = parser.finish_span(start);
        let whole_span = ident.span.start..type_parameters_span.end;

        ASTExpr {
            kind: ASTExprKind::PostfixExpr(
                Box::new(ident),
                ASTPostfixOp {
                    kind: ASTPostfixOpKind::Subscript(type_parameters),
                    span: type_parameters_span,
                },
            ),
            span: whole_span,
        }
    } else {
        ident
    }
}

#[cfg(test)]
mod tests {
    use gdtk_gdscript_ast::*;

    use super::*;
    use crate::test_utils::{create_parser, make_ident, make_number};

    #[test]
    fn test_annotation_empty() {
        let mut parser = create_parser("@annotation");
        let result = parse_annotation(&mut parser);
        let expected = ASTAnnotationStmt {
            identifier: make_ident("annotation"),
            arguments: None,
        };

        assert_eq!(result, expected);
    }

    #[test]
    fn test_annotation_zero_args() {
        let mut parser = create_parser("@annotation()");
        let result = parse_annotation(&mut parser);
        let expected = ASTAnnotationStmt {
            identifier: make_ident("annotation"),
            arguments: Some(vec![]),
        };

        assert_eq!(result, expected);
    }

    #[test]
    fn test_annotation_one_arg() {
        let mut parser = create_parser("@annotation(0)");
        let result = parse_annotation(&mut parser);
        let expected = ASTAnnotationStmt {
            identifier: make_ident("annotation"),
            arguments: Some(vec![make_number(0)]),
        };

        assert_eq!(result, expected);
    }

    #[test]
    fn test_annotation_two_args() {
        let mut parser = create_parser("@annotation(0, 1)");
        let result = parse_annotation(&mut parser);
        let expected = ASTAnnotationStmt {
            identifier: make_ident("annotation"),
            arguments: Some(vec![make_number(0), make_number(1)]),
        };

        assert_eq!(result, expected);
    }

    #[test]
    fn test_annotation_trailing_comma() {
        let mut parser = create_parser("@annotation(0,)");
        let result = parse_annotation(&mut parser);
        let expected = ASTAnnotationStmt {
            identifier: make_ident("annotation"),
            arguments: Some(vec![make_number(0)]),
        };

        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_signal_basic() {
        let mut parser = create_parser("signal done");
        let result = parse_signal(&mut parser);
        let expected = ASTSignalStmt {
            identifier: make_ident("done"),
            parameters: None,
        };

        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_signal_with_parameters() {
        let mut parser = create_parser("signal done(a, b: int)");
        let result = parse_signal(&mut parser);
        let expected = ASTSignalStmt {
            identifier: make_ident("done"),
            parameters: Some(vec![
                ASTVariable {
                    kind: ASTVariableKind::Binding,
                    identifier: make_ident("a"),
                    infer_type: false,
                    typehint: None,
                    value: None,
                    getter: None,
                    setter: None,
                },
                ASTVariable {
                    kind: ASTVariableKind::Binding,
                    identifier: make_ident("b"),
                    infer_type: false,
                    typehint: Some(make_ident("int")),
                    value: None,
                    getter: None,
                    setter: None,
                },
            ]),
        };

        assert_eq!(result, expected);
    }
}
