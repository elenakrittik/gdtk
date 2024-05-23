use gdtk_gdscript_ast::{
    ASTAssertStmt, ASTBreakStmt, ASTBreakpointStmt, ASTClassNameStmt, ASTContinueStmt, ASTElifStmt,
    ASTElseStmt, ASTExpr, ASTExtendsStmt, ASTForStmt, ASTIfStmt, ASTPassStmt, ASTReturnStmt,
    ASTStatement, ASTVariable, ASTVariableKind, ASTWhileStmt, CodeBlock,
};

use crate::lexer::{Token, TokenKind};
use crate::{
    block::parse_block,
    expressions::parse_expr,
    misc::parse_type,
    utils::{expect, parse_ident},
    variables::parse_variable_body,
    Parser,
};

/// Parses a `return` statement.
pub fn parse_return_stmt<'a>(
    parser: &mut Parser<'a, impl Iterator<Item = Token<'a>>>,
) -> ASTStatement<'a> {
    let start = parser.span_start();

    expect!(parser, TokenKind::Return);

    let expr = if parser.peek().is_some_and(|t| !t.kind.is_line_end()) {
        Some(parse_expr(parser))
    } else {
        None
    };

    ASTStatement::Return(ASTReturnStmt {
        expr,
        span: parser.finish_span(start),
    })
}

/// Parses a `for` statement.
pub fn parse_for_stmt<'a>(
    parser: &mut Parser<'a, impl Iterator<Item = Token<'a>>>,
) -> ASTStatement<'a> {
    expect!(parser, TokenKind::For);
    let identifier = parse_ident(parser);

    let typehint = if parser.peek().is_some_and(|t| t.kind.is_colon()) {
        parser.next();
        Some(parse_type(parser))
    } else {
        None
    };

    expect!(parser, TokenKind::In);
    let container = parse_expr(parser);
    expect!(parser, TokenKind::Colon);
    let block = parse_block(parser, false);

    ASTStatement::For(ASTForStmt {
        binding: ASTVariable {
            identifier,
            infer_type: true,
            typehint,
            value: None,
            kind: ASTVariableKind::Binding,
            setter: None,
            getter: None,
        },
        container,
        block,
    })
}

/// Parses a ``class_name`` statement.
pub fn parse_classname_stmt<'a>(
    parser: &mut Parser<'a, impl Iterator<Item = Token<'a>>>,
) -> ASTStatement<'a> {
    let start = parser.span_start();

    expect!(parser, TokenKind::ClassName);

    let identifier = parse_ident(parser);

    ASTStatement::ClassName(ASTClassNameStmt {
        identifier,
        span: parser.finish_span(start),
    })
}

/// Parses an `extends` statement.
pub fn parse_extends_stmt<'a>(
    parser: &mut Parser<'a, impl Iterator<Item = Token<'a>>>,
) -> ASTStatement<'a> {
    let start = parser.span_start();

    expect!(parser, TokenKind::Extends);

    let identifier = parse_ident(parser);

    ASTStatement::Extends(ASTExtendsStmt {
        identifier,
        span: parser.finish_span(start),
    })
}

/// Parses a `var` statement.
pub fn parse_var_stmt<'a>(
    parser: &mut Parser<'a, impl Iterator<Item = Token<'a>>>,
) -> ASTStatement<'a> {
    expect!(parser, TokenKind::Var);
    ASTStatement::Variable(parse_variable_body(parser, ASTVariableKind::Regular))
}

/// Parses a `const` statement.
pub fn parse_const_stmt<'a>(
    parser: &mut Parser<'a, impl Iterator<Item = Token<'a>>>,
) -> ASTStatement<'a> {
    expect!(parser, TokenKind::Const);
    ASTStatement::Variable(parse_variable_body(parser, ASTVariableKind::Constant))
}

/// Parses a `static var` statement.
pub fn parse_static_var_stmt<'a>(
    parser: &mut Parser<'a, impl Iterator<Item = Token<'a>>>,
) -> ASTStatement<'a> {
    if parser.peek().is_some_and(|t| t.kind.is_static()) {
        parser.next();
    }

    expect!(parser, TokenKind::Var);
    ASTStatement::Variable(parse_variable_body(parser, ASTVariableKind::Static))
}

/// Parses an `else` statement.
pub fn parse_else_stmt<'a>(
    parser: &mut Parser<'a, impl Iterator<Item = Token<'a>>>,
) -> ASTStatement<'a> {
    expect!(parser, TokenKind::Else);
    expect!(parser, TokenKind::Colon);
    ASTStatement::Else(ASTElseStmt {
        block: parse_block(parser, false),
    })
}

/// Parses an `if` statement.
pub fn parse_if_stmt<'a>(
    parser: &mut Parser<'a, impl Iterator<Item = Token<'a>>>,
) -> ASTStatement<'a> {
    expect!(parser, TokenKind::If);
    let tuple = parse_iflike(parser);
    ASTStatement::If(ASTIfStmt {
        expr: tuple.0,
        block: tuple.1,
    })
}

/// Parses an `elif` statement.
pub fn parse_elif_stmt<'a>(
    parser: &mut Parser<'a, impl Iterator<Item = Token<'a>>>,
) -> ASTStatement<'a> {
    expect!(parser, TokenKind::Elif);
    let tuple = parse_iflike(parser);
    ASTStatement::Elif(ASTElifStmt {
        expr: tuple.0,
        block: tuple.1,
    })
}

/// Parses a `while` statement.
pub fn parse_while_stmt<'a>(
    parser: &mut Parser<'a, impl Iterator<Item = Token<'a>>>,
) -> ASTStatement<'a> {
    expect!(parser, TokenKind::While);
    let tuple = parse_iflike(parser);
    ASTStatement::While(ASTWhileStmt {
        expr: tuple.0,
        block: tuple.1,
    })
}

/// Parse a
fn parse_iflike<'a>(
    parser: &mut Parser<'a, impl Iterator<Item = Token<'a>>>,
) -> (ASTExpr<'a>, CodeBlock<'a>) {
    let cond = parse_expr(parser);

    expect!(parser, TokenKind::Colon);

    let block = parse_block(parser, false);

    (cond, block)
}

/// Parses a `break` statement.
pub fn parse_break_stmt<'a>(
    parser: &mut Parser<'a, impl Iterator<Item = Token<'a>>>,
) -> ASTStatement<'a> {
    let start = parser.span_start();

    expect!(parser, TokenKind::Break);

    ASTStatement::Break(ASTBreakStmt {
        span: parser.finish_span(start),
    })
}

/// Parses a `breakpoint` statement.
pub fn parse_breakpoint_stmt<'a>(
    parser: &mut Parser<'a, impl Iterator<Item = Token<'a>>>,
) -> ASTStatement<'a> {
    let start = parser.span_start();

    expect!(parser, TokenKind::Breakpoint);

    ASTStatement::Breakpoint(ASTBreakpointStmt {
        span: parser.finish_span(start),
    })
}

/// Parses a `continue` statement.
pub fn parse_continue_stmt<'a>(
    parser: &mut Parser<'a, impl Iterator<Item = Token<'a>>>,
) -> ASTStatement<'a> {
    let start = parser.span_start();

    expect!(parser, TokenKind::Continue);

    ASTStatement::Continue(ASTContinueStmt {
        span: parser.finish_span(start),
    })
}

/// Parses a `pass` statement.
pub fn parse_pass_stmt<'a>(
    parser: &mut Parser<'a, impl Iterator<Item = Token<'a>>>,
) -> ASTStatement<'a> {
    let start = parser.span_start();

    expect!(parser, TokenKind::Pass);

    ASTStatement::Pass(ASTPassStmt {
        span: parser.finish_span(start),
    })
}

/// Parses an `assert` statement.
pub fn parse_assert_stmt<'a>(
    parser: &mut Parser<'a, impl Iterator<Item = Token<'a>>>,
) -> ASTStatement<'a> {
    let start = parser.span_start();

    expect!(parser, TokenKind::Assert);
    let expr = parse_expr(parser);

    ASTStatement::Assert(ASTAssertStmt {
        expr,
        span: parser.finish_span(start),
    })
}

#[cfg(test)]
mod tests {
    use gdtk_gdscript_ast::*;

    use crate::statements::{
        parse_classname_stmt, parse_const_stmt, parse_elif_stmt, parse_else_stmt,
        parse_extends_stmt, parse_for_stmt, parse_if_stmt, parse_return_stmt,
        parse_static_var_stmt, parse_var_stmt, parse_while_stmt,
    };
    use crate::test_utils::{create_parser, make_ident, make_number};

    #[test]
    fn test_var_stmt() {
        let mut parser = create_parser("var a = 1");
        let expected = ASTStatement::Variable(ASTVariable {
            kind: ASTVariableKind::Regular,
            identifier: make_ident("a"),
            infer_type: false,
            typehint: None,
            value: Some(make_number(1)),
            getter: None,
            setter: None,
        });
        let result = parse_var_stmt(&mut parser);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_const_stmt() {
        let mut parser = create_parser("const a = 1");
        let expected = ASTStatement::Variable(ASTVariable {
            kind: ASTVariableKind::Constant,
            identifier: make_ident("a"),
            infer_type: false,
            typehint: None,
            value: Some(make_number(1)),
            getter: None,
            setter: None,
        });
        let result = parse_const_stmt(&mut parser);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_static_var_stmt() {
        let mut parser = create_parser("static var a = 1");
        let expected = ASTStatement::Variable(ASTVariable {
            kind: ASTVariableKind::Static,
            identifier: make_ident("a"),
            infer_type: false,
            typehint: None,
            value: Some(make_number(1)),
            getter: None,
            setter: None,
        });

        parser.next(); // simulate consuming `static`

        let result = parse_static_var_stmt(&mut parser);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_classname_stmt() {
        let mut parser = create_parser("class_name A");
        let expected = ASTStatement::ClassName(ASTClassNameStmt {
            identifier: make_ident("A"),
            span: 0..0,
        });
        let result = parse_classname_stmt(&mut parser);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_extends_stmt() {
        let mut parser = create_parser("extends A");
        let expected = ASTStatement::Extends(ASTExtendsStmt {
            identifier: make_ident("A"),
            span: 0..0,
        });
        let result = parse_extends_stmt(&mut parser);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_return_stmt() {
        let mut parser = create_parser("return 1");
        let expected = ASTStatement::Return(ASTReturnStmt {
            expr: Some(make_number(1)),
            span: 0..0,
        });
        let result = parse_return_stmt(&mut parser);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_return_no_value_stmt() {
        let mut parser = create_parser("return");
        let expected = ASTStatement::Return(ASTReturnStmt {
            expr: None,
            span: 0..0,
        });
        let result = parse_return_stmt(&mut parser);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_elif_stmt() {
        let mut parser = create_parser("elif 1:\n    2");
        let expected = ASTStatement::Elif(ASTElifStmt {
            expr: make_number(1),
            block: vec![ASTStatement::Expr(make_number(2))],
        });
        let result = parse_elif_stmt(&mut parser);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_else_stmt() {
        let mut parser = create_parser("else:\n    2");
        let expected = ASTStatement::Else(ASTElseStmt {
            block: vec![ASTStatement::Expr(make_number(2))],
        });
        let result = parse_else_stmt(&mut parser);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_for_stmt() {
        let mut parser = create_parser("for i in [1, 2]:\n    3");
        let expected = ASTStatement::For(ASTForStmt {
            binding: ASTVariable {
                kind: ASTVariableKind::Binding,
                identifier: make_ident("i"),
                infer_type: true,
                typehint: None,
                value: None,
                getter: None,
                setter: None,
            },
            container: ASTExpr {
                kind: ASTExprKind::Array(vec![make_number(1), make_number(2)]),
                span: 0..0,
            },
            block: vec![ASTStatement::Expr(make_number(3))],
        });
        let result = parse_for_stmt(&mut parser);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_if_stmt() {
        let mut parser = create_parser("if 1:\n    2");
        let expected = ASTStatement::If(ASTIfStmt {
            expr: make_number(1),
            block: vec![ASTStatement::Expr(make_number(2))],
        });
        let result = parse_if_stmt(&mut parser);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_while_stmt() {
        let mut parser = create_parser("while 1:\n    2");
        let expected = ASTStatement::While(ASTWhileStmt {
            expr: make_number(1),
            block: vec![ASTStatement::Expr(make_number(2))],
        });
        let result = parse_while_stmt(&mut parser);

        assert_eq!(result, expected);
    }
}
