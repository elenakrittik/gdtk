use std::iter::Peekable;

use gdtk_ast::poor::{ASTFunction, ASTVariableKind};
use gdtk_lexer::{Token, TokenKind};

use crate::block::parse_block;
use crate::misc::parse_type;
use crate::utils::{delemited_by, expect};
use crate::variables::parse_variable_body;

pub fn parse_func<'a>(
    iter: &mut Peekable<impl Iterator<Item = Token<'a>>>,
    lambda: bool,
) -> ASTFunction<'a> {
    expect!(iter, TokenKind::Func);

    let mut identifier = None;
    let mut return_type = None;

    // Intentionally allow no identifier even when `lambda == false`.
    if iter
        .peek()
        .is_some_and(|t| matches!(t.kind, TokenKind::Identifier(_)))
    {
        identifier = Some(expect!(iter, TokenKind::Identifier(s), s));
    }

    expect!(iter, TokenKind::OpeningParenthesis);

    let parameters = delemited_by(
        iter,
        TokenKind::Comma,
        &[TokenKind::ClosingParenthesis],
        |iter| parse_variable_body(iter, ASTVariableKind::Binding),
    );

    expect!(iter, TokenKind::ClosingParenthesis);

    if iter
        .peek()
        .is_some_and(|t| matches!(t.kind, TokenKind::Arrow))
    {
        iter.next();
        return_type = Some(parse_type(iter));
    }

    expect!(iter, TokenKind::Colon);

    let body = parse_block(iter, lambda);

    ASTFunction {
        identifier,
        parameters,
        return_type: return_type.map(Box::new),
        body,
    }
}

#[cfg(test)]
mod tests {
    use gdtk_ast::poor::*;

    use crate::functions::parse_func;
    use crate::test_utils::create_parser;

    #[test]
    fn test_parse_func_simple() {
        let mut parser = create_parser("func foo(): pass");
        let expected = ASTFunction {
            identifier: Some("foo"),
            parameters: vec![],
            return_type: None,
            body: vec![ASTStatement::Pass],
        };
        let result = parse_func(&mut parser, false);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_func_simple_with_return_type() {
        let mut parser = create_parser("func foo() -> int: pass");
        let expected = ASTFunction {
            identifier: Some("foo"),
            parameters: vec![],
            return_type: Some(Box::new(ASTValue::Identifier("int"))),
            body: vec![ASTStatement::Pass],
        };
        let result = parse_func(&mut parser, false);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_func_unnamed() {
        let mut parser = create_parser("func(): pass");
        let expected = ASTFunction {
            identifier: None,
            parameters: vec![],
            return_type: None,
            body: vec![ASTStatement::Pass],
        };
        let result = parse_func(&mut parser, false);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_func_unnamed_with_return_type() {
        let mut parser = create_parser("func() -> int: pass");
        let expected = ASTFunction {
            identifier: None,
            parameters: vec![],
            return_type: Some(Box::new(ASTValue::Identifier("int"))),
            body: vec![ASTStatement::Pass],
        };
        let result = parse_func(&mut parser, false);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_func_with_parameters() {
        let mut parser = create_parser("func foo(a, b: int, c := 0, d: int = 0): pass");
        let expected = ASTFunction {
            identifier: Some("foo"),
            parameters: vec![
                ASTVariable {
                    identifier: "a",
                    kind: ASTVariableKind::Binding,
                    infer_type: false,
                    typehint: None,
                    value: None,
                },
                ASTVariable {
                    identifier: "b",
                    kind: ASTVariableKind::Binding,
                    infer_type: false,
                    typehint: Some(ASTValue::Identifier("int")),
                    value: None,
                },
                ASTVariable {
                    identifier: "c",
                    kind: ASTVariableKind::Binding,
                    infer_type: true,
                    typehint: None,
                    value: Some(ASTValue::Number(0)),
                },
                ASTVariable {
                    identifier: "d",
                    kind: ASTVariableKind::Binding,
                    infer_type: false,
                    typehint: Some(ASTValue::Identifier("int")),
                    value: Some(ASTValue::Number(0)),
                },
            ],
            return_type: None,
            body: vec![ASTStatement::Pass],
        };
        let result = parse_func(&mut parser, false);

        assert_eq!(result, expected);
    }
}
