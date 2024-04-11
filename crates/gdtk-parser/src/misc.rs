use std::iter::Peekable;

use gdtk_ast::poor::{ASTAnnotation, ASTSignal, ASTVariableKind};
use gdtk_lexer::{Token, TokenKind};

use crate::{
    expressions::parse_expr,
    utils::{delemited_by, expect},
    variables::parse_variable_body,
};

pub fn parse_annotation<'a>(
    iter: &mut Peekable<impl Iterator<Item = Token<'a>>>,
) -> ASTAnnotation<'a> {
    expect!(iter, TokenKind::Annotation);

    let identifier = expect!(iter, TokenKind::Identifier(i), i);

    let arguments = if iter.peek().is_some_and(|t| t.kind.is_opening_parenthesis()) {
        iter.next();

        let args = delemited_by(
            iter,
            TokenKind::Comma,
            &[TokenKind::ClosingParenthesis],
            parse_expr,
        );

        expect!(iter, TokenKind::ClosingParenthesis);

        Some(args)
    } else {
        None
    };

    ASTAnnotation {
        identifier,
        arguments,
    }
}

pub fn parse_signal<'a>(iter: &mut Peekable<impl Iterator<Item = Token<'a>>>) -> ASTSignal<'a> {
    expect!(iter, TokenKind::Signal);

    let identifier = expect!(iter, TokenKind::Identifier(s), s);

    let parameters = if iter.peek().is_some_and(|t| t.kind.is_opening_parenthesis()) {
        iter.next();

        let params = delemited_by(
            iter,
            TokenKind::Comma,
            &[TokenKind::ClosingParenthesis],
            |iter| parse_variable_body(iter, ASTVariableKind::Binding),
        );

        expect!(iter, TokenKind::ClosingParenthesis);

        Some(params)
    } else {
        None
    };

    ASTSignal {
        identifier,
        parameters,
    }
}

#[cfg(test)]
mod tests {
    use gdtk_ast::poor::*;

    use super::*;
    use crate::test_utils::create_parser;

    #[test]
    fn test_annotation_empty() {
        let mut parser = create_parser("@annotation");
        let result = parse_annotation(&mut parser);
        let expected = ASTAnnotation {
            identifier: "annotation",
            arguments: None,
        };

        assert_eq!(result, expected);
    }

    #[test]
    fn test_annotation_zero_args() {
        let mut parser = create_parser("@annotation()");
        let result = parse_annotation(&mut parser);
        let expected = ASTAnnotation {
            identifier: "annotation",
            arguments: Some(vec![]),
        };

        assert_eq!(result, expected);
    }

    #[test]
    fn test_annotation_one_arg() {
        let mut parser = create_parser("@annotation(0)");
        let result = parse_annotation(&mut parser);
        let expected = ASTAnnotation {
            identifier: "annotation",
            arguments: Some(vec![ASTValue::Number(0)]),
        };

        assert_eq!(result, expected);
    }

    #[test]
    fn test_annotation_two_args() {
        let mut parser = create_parser("@annotation(0, 1)");
        let result = parse_annotation(&mut parser);
        let expected = ASTAnnotation {
            identifier: "annotation",
            arguments: Some(vec![ASTValue::Number(0), ASTValue::Number(1)]),
        };

        assert_eq!(result, expected);
    }

    #[test]
    fn test_annotation_trailing_comma() {
        let mut parser = create_parser("@annotation(0,)");
        let result = parse_annotation(&mut parser);
        let expected = ASTAnnotation {
            identifier: "annotation",
            arguments: Some(vec![ASTValue::Number(0)]),
        };

        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_signal_basic() {
        let mut parser = create_parser("signal done");
        let result = parse_signal(&mut parser);
        let expected = ASTSignal {
            identifier: "done",
            parameters: None,
        };

        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_signal_with_parameters() {
        let mut parser = create_parser("signal done(a, b: int)");
        let result = parse_signal(&mut parser);
        let expected = ASTSignal {
            identifier: "done",
            parameters: Some(vec![
                ASTVariable {
                    kind: ASTVariableKind::Binding,
                    identifier: "a",
                    infer_type: false,
                    typehint: None,
                    value: None,
                },
                ASTVariable {
                    kind: ASTVariableKind::Binding,
                    identifier: "b",
                    infer_type: false,
                    typehint: Some(ASTValue::Identifier("int")),
                    value: None,
                },
            ]),
        };

        assert_eq!(result, expected);
    }
}
