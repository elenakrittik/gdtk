use gdtk_ast::{ASTVariable, ASTVariableKind};
use gdtk_lexer::{Token, TokenKind};

use crate::expressions::parse_expr;
use crate::misc::parse_type;
use crate::utils::parse_ident;
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

                    if let Some(TokenKind::Assignment) = parser.peek().map(|t| &t.kind) {
                        parser.next();

                        value = Some(parse_expr(parser));
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
    }
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
        };

        assert_eq!(result, expected);
    }
}
