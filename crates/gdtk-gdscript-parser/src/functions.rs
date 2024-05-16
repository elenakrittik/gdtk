use gdtk_ast::{ASTFunction, ASTFunctionKind, ASTVariableKind};
use crate::lexer::{Token, TokenKind};

use crate::block::parse_block;
use crate::misc::parse_type;
use crate::utils::{delemited_by, expect, parse_ident};
use crate::variables::parse_variable_body;
use crate::Parser;

#[derive(Copy, Clone)]
pub struct ParseFuncOptions {
    pub kind: ASTFunctionKind,
    pub is_lambda: bool,
}

pub fn parse_func<'a>(
    parser: &mut Parser<impl Iterator<Item = Token<'a>>>,
    options: ParseFuncOptions,
) -> ASTFunction<'a> {
    let start = parser.span_start();

    if parser.peek().is_some_and(|t| t.kind.is_static()) {
        parser.next();
    }

    if parser.peek().is_some_and(|t| t.kind.is_func()) {
        parser.next();
    }

    let mut identifier = None;
    let mut return_type = None;

    // Intentionally allow no identifier even when `lambda == false`.
    if parser
        .peek()
        .is_some_and(|t| matches!(t.kind, TokenKind::Identifier(_)))
    {
        identifier = Some(Box::new(parse_ident(parser)));
    }

    let parameters = if parser
        .peek()
        .is_some_and(|t| t.kind.is_opening_parenthesis())
    {
        expect!(parser, TokenKind::OpeningParenthesis);

        let parameters = parser.with_parens_ctx(true, |parser| {
            delemited_by(
                parser,
                TokenKind::Comma,
                &[TokenKind::ClosingParenthesis],
                |iter| parse_variable_body(iter, ASTVariableKind::Binding),
            )
        });

        expect!(parser, TokenKind::ClosingParenthesis);

        Some(parameters)
    } else {
        None
    };

    if parser
        .peek()
        .is_some_and(|t| matches!(t.kind, TokenKind::Arrow))
    {
        parser.next();
        return_type = Some(parse_type(parser));
    }

    expect!(parser, TokenKind::Colon);

    let body = parse_block(parser, options.is_lambda);

    ASTFunction {
        identifier,
        parameters,
        return_type: return_type.map(Box::new),
        kind: options.kind,
        body,
        span: parser.finish_span(start),
    }
}

#[cfg(test)]
mod tests {
    use gdtk_ast::*;

    use crate::functions::{parse_func, ParseFuncOptions};
    use crate::test_utils::{create_parser, make_ident, make_number, PASS_STMT};

    #[test]
    fn test_parse_func_simple() {
        let mut parser = create_parser("func foo(): pass");
        let expected = ASTFunction {
            identifier: Some(Box::new(make_ident("foo"))),
            parameters: Some(vec![]),
            return_type: None,
            body: vec![PASS_STMT],
            kind: ASTFunctionKind::Regular,
            span: 0..0,
        };
        let result = parse_func(
            &mut parser,
            ParseFuncOptions {
                kind: ASTFunctionKind::Regular,
                is_lambda: false,
            },
        );

        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_func_simple_with_return_type() {
        let mut parser = create_parser("func foo() -> int: pass");
        let expected = ASTFunction {
            identifier: Some(Box::new(make_ident("foo"))),
            parameters: Some(vec![]),
            return_type: Some(Box::new(make_ident("int"))),
            kind: ASTFunctionKind::Regular,
            body: vec![PASS_STMT],
            span: 0..0,
        };
        let result = parse_func(
            &mut parser,
            ParseFuncOptions {
                kind: ASTFunctionKind::Regular,
                is_lambda: false,
            },
        );

        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_func_unnamed() {
        let mut parser = create_parser("func(): pass");
        let expected = ASTFunction {
            identifier: None,
            parameters: Some(vec![]),
            return_type: None,
            kind: ASTFunctionKind::Regular,
            body: vec![PASS_STMT],
            span: 0..0,
        };
        let result = parse_func(
            &mut parser,
            ParseFuncOptions {
                kind: ASTFunctionKind::Regular,
                is_lambda: false,
            },
        );

        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_func_unnamed_with_return_type() {
        let mut parser = create_parser("func() -> int: pass");
        let expected = ASTFunction {
            identifier: None,
            parameters: Some(vec![]),
            return_type: Some(Box::new(make_ident("int"))),
            kind: ASTFunctionKind::Regular,
            body: vec![PASS_STMT],
            span: 0..0,
        };
        let result = parse_func(
            &mut parser,
            ParseFuncOptions {
                kind: ASTFunctionKind::Regular,
                is_lambda: false,
            },
        );

        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_func_with_parameters() {
        let mut parser = create_parser("func foo(a, b: int, c := 0, d: int = 0): pass");
        let expected = ASTFunction {
            identifier: Some(Box::new(make_ident("foo"))),
            parameters: Some(vec![
                ASTVariable {
                    identifier: make_ident("a"),
                    kind: ASTVariableKind::Binding,
                    infer_type: false,
                    typehint: None,
                    value: None,
                    getter: None,
                    setter: None,
                },
                ASTVariable {
                    identifier: make_ident("b"),
                    kind: ASTVariableKind::Binding,
                    infer_type: false,
                    typehint: Some(make_ident("int")),
                    value: None,
                    getter: None,
                    setter: None,
                },
                ASTVariable {
                    identifier: make_ident("c"),
                    kind: ASTVariableKind::Binding,
                    infer_type: true,
                    typehint: None,
                    value: Some(make_number(0)),
                    getter: None,
                    setter: None,
                },
                ASTVariable {
                    identifier: make_ident("d"),
                    kind: ASTVariableKind::Binding,
                    infer_type: false,
                    typehint: Some(make_ident("int")),
                    value: Some(make_number(0)),
                    getter: None,
                    setter: None,
                },
            ]),
            return_type: None,
            kind: ASTFunctionKind::Regular,
            body: vec![PASS_STMT],
            span: 0..0,
        };
        let result = parse_func(
            &mut parser,
            ParseFuncOptions {
                kind: ASTFunctionKind::Regular,
                is_lambda: false,
            },
        );

        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_static_func() {
        let mut parser = create_parser("static func foo(): pass");
        let expected = ASTFunction {
            identifier: Some(Box::new(make_ident("foo"))),
            parameters: Some(vec![]),
            return_type: None,
            kind: ASTFunctionKind::Static,
            body: vec![PASS_STMT],
            span: 0..0,
        };
        let result = parse_func(
            &mut parser,
            ParseFuncOptions {
                kind: ASTFunctionKind::Static,
                is_lambda: false,
            },
        );

        assert_eq!(result, expected);
    }
}
