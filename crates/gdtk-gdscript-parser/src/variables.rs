use gdtk_ast::{ASTFunction, ASTFunctionKind, ASTVariable, ASTVariableKind};
use crate::lexer::{Token, TokenKind};

use crate::expressions::parse_expr;
use crate::functions::{parse_func, ParseFuncOptions};
use crate::misc::parse_type;
use crate::utils::{expect, parse_ident};
use crate::Parser;

/// Parses variable body, i.e. any variable without preceding keywords.
pub fn parse_variable_body<'a>(
    parser: &mut Parser<'a, impl Iterator<Item = Token<'a>>>,
    kind: ASTVariableKind,
) -> ASTVariable<'a> {
    let identifier = parse_ident(parser);
    let mut typehint = None;
    let mut infer_type = false;
    let mut value = None;
    let mut getter = None;
    let mut setter = None;

    // Possible cases:
    // [var] ident
    // [var] ident = val
    // [var] ident := val
    // [var] ident: type = val
    // [var] ident: type

    match parser.peek().map(|t| &t.kind) {
        Some(TokenKind::Colon) => {
            parser.next();

            match parser.peek().expect("unexpected EOF").kind {
                TokenKind::Assignment => {
                    parser.next();
                    infer_type = true;
                    value = Some(parse_expr(parser));
                }
                TokenKind::Newline => {
                    (getter, setter) =
                        parser.with_parens_ctx(false, |parser| parse_variable_etters(parser));
                }
                _ => {
                    typehint = Some(parse_type(parser));

                    match parser.peek().map(|t| &t.kind) {
                        Some(TokenKind::Assignment) => {
                            parser.next();

                            value = Some(parse_expr(parser));
                        }
                        Some(TokenKind::Colon) => {
                            parser.next();

                            (getter, setter) = parser
                                .with_parens_ctx(false, |parser| parse_variable_etters(parser));
                        }
                        _ => (),
                    }
                }
            };
        }
        Some(TokenKind::Assignment) => {
            parser.next();
            value = Some(parse_expr(parser))
        }
        _ => (),
    }

    ASTVariable {
        identifier,
        infer_type,
        typehint,
        value,
        kind,
        getter,
        setter,
    }
}

fn parse_variable_etters<'a>(
    parser: &mut Parser<'a, impl Iterator<Item = Token<'a>>>,
) -> (Option<ASTFunction<'a>>, Option<ASTFunction<'a>>) {
    expect!(parser, TokenKind::Newline);
    expect!(parser, TokenKind::Indent);

    const OPTIONS: ParseFuncOptions = ParseFuncOptions {
        kind: ASTFunctionKind::Regular,
        is_lambda: false,
    };

    let mut getter = None;
    let mut setter = None;

    while let Some(Token {
        kind: TokenKind::Identifier(ident),
        ..
    }) = parser.peek()
    {
        match *ident {
            "get" => {
                let old = getter.replace(parse_func(parser, OPTIONS));

                if old.is_some() {
                    panic!("variables can only have one getter");
                }
            }
            "set" => {
                let old = setter.replace(parse_func(parser, OPTIONS));

                if old.is_some() {
                    panic!("variables can only have one setter");
                }
            }
            _ => panic!("only 'get' and 'set' are valid associated function names"),
        }
    }

    expect!(parser, TokenKind::Dedent);

    (getter, setter)
}

#[cfg(test)]
mod tests {
    use gdtk_ast::*;

    use crate::test_utils::{create_parser, make_ident, make_number, PASS_STMT};
    use crate::variables::parse_variable_body;

    #[test]
    fn test_variable_empty() {
        let mut parser = create_parser("ident");
        let result = parse_variable_body(&mut parser, ASTVariableKind::Regular);
        let expected = ASTVariable {
            identifier: make_ident("ident"),
            infer_type: false,
            typehint: None,
            value: None,
            kind: ASTVariableKind::Regular,
            getter: None,
            setter: None,
        };

        assert_eq!(result, expected);
    }

    #[test]
    fn test_variable_with_type() {
        let mut parser = create_parser("ident: type");
        let result = parse_variable_body(&mut parser, ASTVariableKind::Regular);
        let expected = ASTVariable {
            identifier: make_ident("ident"),
            infer_type: false,
            typehint: Some(make_ident("type")),
            value: None,
            kind: ASTVariableKind::Regular,
            getter: None,
            setter: None,
        };

        assert_eq!(result, expected);
    }

    #[test]
    fn test_variable_with_value() {
        let mut parser = create_parser("ident = 0");
        let result = parse_variable_body(&mut parser, ASTVariableKind::Regular);
        let expected = ASTVariable {
            identifier: make_ident("ident"),
            infer_type: false,
            typehint: None,
            value: Some(make_number(0)),
            kind: ASTVariableKind::Regular,
            getter: None,
            setter: None,
        };

        assert_eq!(result, expected);
    }

    #[test]
    fn test_variable_with_type_inference_and_value() {
        let mut parser = create_parser("ident := 0");
        let result = parse_variable_body(&mut parser, ASTVariableKind::Regular);
        let expected = ASTVariable {
            identifier: make_ident("ident"),
            infer_type: true,
            typehint: None,
            value: Some(make_number(0)),
            kind: ASTVariableKind::Regular,
            getter: None,
            setter: None,
        };

        assert_eq!(result, expected);
    }

    #[test]
    fn test_variable_with_type_and_value() {
        let mut parser = create_parser("ident: type = 0");
        let result = parse_variable_body(&mut parser, ASTVariableKind::Regular);
        let expected = ASTVariable {
            identifier: make_ident("ident"),
            infer_type: false,
            typehint: Some(make_ident("type")),
            value: Some(make_number(0)),
            kind: ASTVariableKind::Regular,
            getter: None,
            setter: None,
        };

        assert_eq!(result, expected);
    }

    #[test]
    fn test_untyped_variable_with_etter() {
        let mut parser = create_parser("ident:\n    get:\n        pass");
        let result = parse_variable_body(&mut parser, ASTVariableKind::Regular);
        let expected = ASTVariable {
            identifier: make_ident("ident"),
            infer_type: false,
            typehint: None,
            value: None,
            kind: ASTVariableKind::Regular,
            getter: Some(ASTFunction {
                identifier: Some(Box::new(make_ident("get"))),
                parameters: None,
                return_type: None,
                kind: ASTFunctionKind::Regular,
                body: vec![PASS_STMT],
                span: 0..0,
            }),
            setter: None,
        };

        assert_eq!(result, expected);
    }

    #[test]
    fn test_typed_variable_with_etter() {
        let mut parser = create_parser("ident: type:\n    get:\n        pass");
        let result = parse_variable_body(&mut parser, ASTVariableKind::Regular);
        let expected = ASTVariable {
            identifier: make_ident("ident"),
            infer_type: false,
            typehint: Some(make_ident("type")),
            value: None,
            kind: ASTVariableKind::Regular,
            getter: Some(ASTFunction {
                identifier: Some(Box::new(make_ident("get"))),
                parameters: None,
                return_type: None,
                kind: ASTFunctionKind::Regular,
                body: vec![PASS_STMT],
                span: 0..0,
            }),
            setter: None,
        };

        assert_eq!(result, expected);
    }

    #[test]
    fn test_variable_with_multiple_etters() {
        let mut parser = create_parser("ident:\n    get:\n        pass\n    set(x):\n        pass");
        let result = parse_variable_body(&mut parser, ASTVariableKind::Regular);
        let expected = ASTVariable {
            identifier: make_ident("ident"),
            infer_type: false,
            typehint: None,
            value: None,
            kind: ASTVariableKind::Regular,
            getter: Some(ASTFunction {
                identifier: Some(Box::new(make_ident("get"))),
                parameters: None,
                return_type: None,
                kind: ASTFunctionKind::Regular,
                body: vec![PASS_STMT],
                span: 0..0,
            }),
            setter: Some(ASTFunction {
                identifier: Some(Box::new(make_ident("set"))),
                parameters: Some(vec![ASTVariable {
                    identifier: make_ident("x"),
                    infer_type: false,
                    typehint: None,
                    value: None,
                    kind: ASTVariableKind::Binding,
                    getter: None,
                    setter: None,
                }]),
                return_type: None,
                kind: ASTFunctionKind::Regular,
                body: vec![PASS_STMT],
                span: 0..0,
            }),
        };

        assert_eq!(result, expected);
    }
}
