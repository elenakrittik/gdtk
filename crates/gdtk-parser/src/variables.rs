use std::iter::Peekable;

use gdtk_ast::poor::{ASTVariable, ASTVariableKind};
use gdtk_lexer::{Token, TokenKind};

use crate::expressions::parse_expr;
use crate::utils::{expect, next_non_blank, peek_non_blank};

/// Parses variable body, i.e. any variable without preceding keywords.
pub fn parse_variable_body<'a>(
    iter: &mut Peekable<impl Iterator<Item = Token<'a>>>,
    kind: ASTVariableKind,
) -> ASTVariable<'a> {
    let identifier = expect!(iter, TokenKind::Identifier(s), s);
    let mut typehint = None;
    let mut infer_type = false;
    let mut value = None;

    // Possible cases:
    // [var] ident
    // [var] ident = val
    // [var] ident := val
    // [var] ident: type = val
    // [var] ident: type

    if peek_non_blank(iter).is_some_and(|t| t.kind.is_colon() || t.kind.is_assignment()) {
        match peek_non_blank(iter).expect("unexpected EOF").kind {
            TokenKind::Colon => {
                iter.next();

                match peek_non_blank(iter).expect("unexpected EOF").kind {
                    TokenKind::Assignment => {
                        iter.next();
                        infer_type = true;
                        value = Some(parse_expr(iter));
                    }
                    _ => {
                        typehint = Some(parse_expr(iter));

                        if peek_non_blank(iter).is_some_and(|t| t.kind.is_assignment()) {
                            match next_non_blank(iter) {
                                Token {
                                    kind: TokenKind::Assignment,
                                    ..
                                } => value = Some(parse_expr(iter)),
                                _ => unreachable!(),
                            }
                        }
                    }
                };
            }
            TokenKind::Assignment => {
                iter.next();
                value = Some(parse_expr(iter))
            }
            _ => unreachable!(),
        }
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
    use gdtk_ast::poor::*;

    use crate::test_utils::create_parser;

    #[test]
    fn test_variable_empty() {
        let mut parser = create_parser("ident");
        let result = super::parse_variable_body(&mut parser, ASTVariableKind::Regular);
        let expected = ASTVariable {
            identifier: "ident",
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
        let result = super::parse_variable_body(&mut parser, ASTVariableKind::Regular);
        let expected = ASTVariable {
            identifier: "ident",
            infer_type: false,
            typehint: Some(ASTValue::Identifier("type")),
            value: None,
            kind: ASTVariableKind::Regular,
        };

        assert_eq!(result, expected);
    }

    #[test]
    fn test_variable_with_value() {
        let mut parser = create_parser("ident = 0");
        let result = super::parse_variable_body(&mut parser, ASTVariableKind::Regular);
        let expected = ASTVariable {
            identifier: "ident",
            infer_type: false,
            typehint: None,
            value: Some(ASTValue::Number(0)),
            kind: ASTVariableKind::Regular,
        };

        assert_eq!(result, expected);
    }

    #[test]
    fn test_variable_with_type_inference_and_value() {
        let mut parser = create_parser("ident := 0");
        let result = super::parse_variable_body(&mut parser, ASTVariableKind::Regular);
        let expected = ASTVariable {
            identifier: "ident",
            infer_type: true,
            typehint: None,
            value: Some(ASTValue::Number(0)),
            kind: ASTVariableKind::Regular,
        };

        assert_eq!(result, expected);
    }

    #[test]
    fn test_variable_with_type_and_value() {
        let mut parser = create_parser("ident: type = 0");
        let result = super::parse_variable_body(&mut parser, ASTVariableKind::Regular);
        let expected = ASTVariable {
            identifier: "ident",
            infer_type: false,
            typehint: Some(ASTValue::Identifier("type")),
            value: Some(ASTValue::Number(0)),
            kind: ASTVariableKind::Regular,
        };

        assert_eq!(result, expected);
    }
}
