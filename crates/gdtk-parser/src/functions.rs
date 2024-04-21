use gdtk_ast::{ASTFunction, ASTFunctionKind, ASTVariableKind};
use gdtk_lexer::{Token, TokenKind};

use crate::block::parse_block;
use crate::misc::parse_type;
use crate::utils::{delemited_by, expect};
use crate::variables::parse_variable_body;
use crate::Parser;

pub fn parse_func<'a>(
    parser: &mut Parser<impl Iterator<Item = Token<'a>>>,
    kind: ASTFunctionKind,
    lambda: bool,
) -> ASTFunction<'a> {
    if parser.peek().is_some_and(|t| t.kind.is_static()) {
        parser.next();
    }

    expect!(parser, TokenKind::Func);

    let mut identifier = None;
    let mut return_type = None;

    // Intentionally allow no identifier even when `lambda == false`.
    if parser
        .peek()
        .is_some_and(|t| matches!(t.kind, TokenKind::Identifier(_)))
    {
        identifier = Some(expect!(parser, TokenKind::Identifier(s), s));
    }

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

    if parser
        .peek()
        .is_some_and(|t| matches!(t.kind, TokenKind::Arrow))
    {
        parser.next();
        return_type = Some(parse_type(parser));
    }

    expect!(parser, TokenKind::Colon);

    let body = parse_block(parser, lambda);

    ASTFunction {
        identifier,
        parameters,
        return_type: return_type.map(Box::new),
        kind,
        body,
    }
}

#[cfg(test)]
mod tests {
    use gdtk_ast::*;

    use crate::functions::parse_func;
    use crate::test_utils::create_parser;

    #[test]
    fn test_parse_func_simple() {
        let mut parser = create_parser("func foo(): pass");
        let expected = ASTFunction {
            identifier: Some("foo"),
            parameters: vec![],
            return_type: None,
            kind: ASTFunctionKind::Regular,
            body: vec![ASTStatement::Pass],
        };
        let result = parse_func(&mut parser, ASTFunctionKind::Regular, false);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_func_simple_with_return_type() {
        let mut parser = create_parser("func foo() -> int: pass");
        let expected = ASTFunction {
            identifier: Some("foo"),
            parameters: vec![],
            return_type: Some(Box::new(ASTExpr::Identifier("int"))),
            kind: ASTFunctionKind::Regular,
            body: vec![ASTStatement::Pass],
        };
        let result = parse_func(&mut parser, ASTFunctionKind::Regular, false);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_func_unnamed() {
        let mut parser = create_parser("func(): pass");
        let expected = ASTFunction {
            identifier: None,
            parameters: vec![],
            return_type: None,
            kind: ASTFunctionKind::Regular,
            body: vec![ASTStatement::Pass],
        };
        let result = parse_func(&mut parser, ASTFunctionKind::Regular, false);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_func_unnamed_with_return_type() {
        let mut parser = create_parser("func() -> int: pass");
        let expected = ASTFunction {
            identifier: None,
            parameters: vec![],
            return_type: Some(Box::new(ASTExpr::Identifier("int"))),
            kind: ASTFunctionKind::Regular,
            body: vec![ASTStatement::Pass],
        };
        let result = parse_func(&mut parser, ASTFunctionKind::Regular, false);

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
                    typehint: Some(ASTExpr::Identifier("int")),
                    value: None,
                },
                ASTVariable {
                    identifier: "c",
                    kind: ASTVariableKind::Binding,
                    infer_type: true,
                    typehint: None,
                    value: Some(ASTExpr::Number(0)),
                },
                ASTVariable {
                    identifier: "d",
                    kind: ASTVariableKind::Binding,
                    infer_type: false,
                    typehint: Some(ASTExpr::Identifier("int")),
                    value: Some(ASTExpr::Number(0)),
                },
            ],
            return_type: None,
            kind: ASTFunctionKind::Regular,
            body: vec![ASTStatement::Pass],
        };
        let result = parse_func(&mut parser, ASTFunctionKind::Regular, false);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_static_func() {
        let mut parser = create_parser("static func foo(): pass");
        let expected = ASTFunction {
            identifier: Some("foo"),
            parameters: vec![],
            return_type: None,
            kind: ASTFunctionKind::Static,
            body: vec![ASTStatement::Pass],
        };
        let result = parse_func(&mut parser, ASTFunctionKind::Static, false);

        assert_eq!(result, expected);
    }
}
