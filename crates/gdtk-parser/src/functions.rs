use gdtk_ast::{ASTFunction, ASTVariableKind};
use gdtk_lexer::{Token, TokenKind};

use crate::block::parse_block;
use crate::misc::parse_type;
use crate::utils::{delemited_by, expect, parse_ident};
use crate::variables::parse_variable_body;
use crate::Parser;

pub fn parse_func<'a>(
    parser: &mut Parser<'a, impl Iterator<Item = Token<'a>>>,
    lambda: bool,
) -> ASTFunction<'a> {
    let start = parser.range_start();

    expect!(parser, TokenKind::Func);

    let mut identifier = None;
    let mut return_type = None;

    // Intentionally allow no identifier even when `lambda == false`.
    if parser
        .peek()
        .is_some_and(|t| matches!(t.kind, TokenKind::Identifier(_)))
    {
        identifier = Some(Box::new(parse_ident(parser)));
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
        body,
        range: parser.finish_range(start),
    }
}

#[cfg(test)]
mod tests {
    use gdtk_ast::*;

    use crate::functions::parse_func;
    use crate::test_utils::{create_parser, make_ident, make_number};

    const PASS_STMT: ASTStatement = ASTStatement::Pass(ASTPassStmt { range: 0..0 });

    #[test]
    fn test_parse_func_simple() {
        let mut parser = create_parser("func foo(): pass");
        let expected = ASTFunction {
            identifier: Some(Box::new(make_ident("foo"))),
            parameters: vec![],
            return_type: None,
            body: vec![PASS_STMT],
            range: 0..0,
        };
        let result = parse_func(&mut parser, false);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_func_simple_with_return_type() {
        let mut parser = create_parser("func foo() -> int: pass");
        let expected = ASTFunction {
            identifier: Some(Box::new(make_ident("foo"))),
            parameters: vec![],
            return_type: Some(Box::new(make_ident("int"))),
            body: vec![PASS_STMT],
            range: 0..0,
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
            body: vec![PASS_STMT],
            range: 0..0,
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
            return_type: Some(Box::new(make_ident("int"))),
            body: vec![PASS_STMT],
            range: 0..0,
        };
        let result = parse_func(&mut parser, false);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_func_with_parameters() {
        let mut parser = create_parser("func foo(a, b: int, c := 0, d: int = 0): pass");
        let expected = ASTFunction {
            identifier: Some(Box::new(make_ident("foo"))),
            parameters: vec![
                ASTVariable {
                    identifier: make_ident("a"),
                    kind: ASTVariableKind::Binding,
                    infer_type: false,
                    typehint: None,
                    value: None,
                },
                ASTVariable {
                    identifier: make_ident("b"),
                    kind: ASTVariableKind::Binding,
                    infer_type: false,
                    typehint: Some(make_ident("int")),
                    value: None,
                },
                ASTVariable {
                    identifier: make_ident("c"),
                    kind: ASTVariableKind::Binding,
                    infer_type: true,
                    typehint: None,
                    value: Some(make_number(0)),
                },
                ASTVariable {
                    identifier: make_ident("d"),
                    kind: ASTVariableKind::Binding,
                    infer_type: false,
                    typehint: Some(make_ident("int")),
                    value: Some(make_number(0)),
                },
            ],
            return_type: None,
            body: vec![PASS_STMT],
            range: 0..0,
        };
        let result = parse_func(&mut parser, false);

        assert_eq!(result, expected);
    }
}
