use gdtk_ast::{ASTFunction, ASTFunctionKind, ASTVariable, ASTVariableKind};
use gdtk_lexer::{Token, TokenKind};

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
            "get" => getter = Some(parse_func(parser, OPTIONS)),
            "set" => setter = Some(parse_func(parser, OPTIONS)),
            _ => panic!("only 'get' and 'set' are valid associated function names"),
        }
    }

    expect!(parser, TokenKind::Dedent);

    (getter, setter)
}

#[cfg(test)]
mod tests {
    use gdtk_ast::*;

    use crate::test_utils::{create_parser, make_ident, make_number};
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
}
