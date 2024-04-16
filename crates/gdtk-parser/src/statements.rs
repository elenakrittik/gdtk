use gdtk_ast::{
    ASTElifStmt, ASTElseStmt, ASTExpr, ASTForStmt, ASTIfStmt, ASTStatement, ASTVariable,
    ASTVariableKind, ASTWhileStmt, CodeBlock,
};
use gdtk_lexer::{Token, TokenKind};

use crate::{
    block::parse_block, expressions::parse_expr, misc::parse_type, utils::expect,
    variables::parse_variable_body, Parser,
};

/// Parses a `return` statement.
pub fn parse_return_stmt<'a>(
    parser: &mut Parser<impl Iterator<Item = Token<'a>>>,
) -> ASTStatement<'a> {
    expect!(parser, TokenKind::Return);

    let value = if parser.peek().is_some_and(|t| !t.kind.is_line_end()) {
        Some(parse_expr(parser))
    } else {
        None
    };

    ASTStatement::Return(value)
}

/// Parses a `for` statement.
pub fn parse_for_stmt<'a>(
    parser: &mut Parser<impl Iterator<Item = Token<'a>>>,
) -> ASTStatement<'a> {
    expect!(parser, TokenKind::For);
    let identifier = expect!(parser, TokenKind::Identifier(s), s);

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
        },
        container,
        block,
    })
}

/// Parses a ``class_name`` statement.
pub fn parse_classname_stmt<'a>(
    parser: &mut Parser<impl Iterator<Item = Token<'a>>>,
) -> ASTStatement<'a> {
    expect!(parser, TokenKind::ClassName);
    expect!(parser, TokenKind::Identifier(i), ASTStatement::ClassName(i))
}

/// Parses an `extends` statement.
pub fn parse_extends_stmt<'a>(
    parser: &mut Parser<impl Iterator<Item = Token<'a>>>,
) -> ASTStatement<'a> {
    expect!(parser, TokenKind::Extends);
    expect!(parser, TokenKind::Identifier(i), ASTStatement::Extends(i))
}

/// Parses a `var` statement.
pub fn parse_var_stmt<'a>(
    parser: &mut Parser<impl Iterator<Item = Token<'a>>>,
) -> ASTStatement<'a> {
    expect!(parser, TokenKind::Var);
    ASTStatement::Variable(parse_variable_body(parser, ASTVariableKind::Regular))
}

/// Parses a `const` statement.
pub fn parse_const_stmt<'a>(
    parser: &mut Parser<impl Iterator<Item = Token<'a>>>,
) -> ASTStatement<'a> {
    expect!(parser, TokenKind::Const);
    ASTStatement::Variable(parse_variable_body(parser, ASTVariableKind::Constant))
}

/// Parses a `static var` statement.
pub fn parse_static_var_stmt<'a>(
    parser: &mut Parser<impl Iterator<Item = Token<'a>>>,
) -> ASTStatement<'a> {
    expect!(parser, TokenKind::Static);
    expect!(parser, TokenKind::Var);
    ASTStatement::Variable(parse_variable_body(parser, ASTVariableKind::Static))
}

/// Parses an `else` statement.
pub fn parse_else_stmt<'a>(
    parser: &mut Parser<impl Iterator<Item = Token<'a>>>,
) -> ASTStatement<'a> {
    expect!(parser, TokenKind::Else);
    expect!(parser, TokenKind::Colon);
    ASTStatement::Else(ASTElseStmt {
        block: parse_block(parser, false),
    })
}

/// Parses an `if` statement.
pub fn parse_if_stmt<'a>(parser: &mut Parser<impl Iterator<Item = Token<'a>>>) -> ASTStatement<'a> {
    expect!(parser, TokenKind::If);
    let tuple = parse_iflike(parser);
    ASTStatement::If(ASTIfStmt {
        expr: tuple.0,
        block: tuple.1,
    })
}

/// Parses an `elif` statement.
pub fn parse_elif_stmt<'a>(
    parser: &mut Parser<impl Iterator<Item = Token<'a>>>,
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
    parser: &mut Parser<impl Iterator<Item = Token<'a>>>,
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
    parser: &mut Parser<impl Iterator<Item = Token<'a>>>,
) -> (ASTExpr<'a>, CodeBlock<'a>) {
    let cond = parse_expr(parser);
    expect!(parser, TokenKind::Colon);
    let block = parse_block(parser, false);

    (cond, block)
}

#[cfg(test)]
mod tests {
    use gdtk_ast::*;

    use crate::statements::{
        parse_classname_stmt, parse_const_stmt, parse_elif_stmt, parse_else_stmt,
        parse_extends_stmt, parse_for_stmt, parse_if_stmt, parse_return_stmt,
        parse_static_var_stmt, parse_var_stmt, parse_while_stmt,
    };
    use crate::test_utils::create_parser;

    #[test]
    fn test_var_stmt() {
        let mut parser = create_parser("var a = 1");
        let expected = ASTStatement::Variable(ASTVariable {
            kind: ASTVariableKind::Regular,
            identifier: "a",
            infer_type: false,
            typehint: None,
            value: Some(ASTExpr::Number(1)),
        });
        let result = parse_var_stmt(&mut parser);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_const_stmt() {
        let mut parser = create_parser("const a = 1");
        let expected = ASTStatement::Variable(ASTVariable {
            kind: ASTVariableKind::Constant,
            identifier: "a",
            infer_type: false,
            typehint: None,
            value: Some(ASTExpr::Number(1)),
        });
        let result = parse_const_stmt(&mut parser);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_static_var_stmt() {
        let mut parser = create_parser("static var a = 1");
        let expected = ASTStatement::Variable(ASTVariable {
            kind: ASTVariableKind::Static,
            identifier: "a",
            infer_type: false,
            typehint: None,
            value: Some(ASTExpr::Number(1)),
        });
        let result = parse_static_var_stmt(&mut parser);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_classname_stmt() {
        let mut parser = create_parser("class_name A");
        let expected = ASTStatement::ClassName("A");
        let result = parse_classname_stmt(&mut parser);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_extends_stmt() {
        let mut parser = create_parser("extends A");
        let expected = ASTStatement::Extends("A");
        let result = parse_extends_stmt(&mut parser);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_return_stmt() {
        let mut parser = create_parser("return 1");
        let expected = ASTStatement::Return(Some(ASTExpr::Number(1)));
        let result = parse_return_stmt(&mut parser);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_return_no_value_stmt() {
        let mut parser = create_parser("return");
        let expected = ASTStatement::Return(None);
        let result = parse_return_stmt(&mut parser);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_elif_stmt() {
        let mut parser = create_parser("elif 1:\n    2");
        let expected = ASTStatement::Elif(ASTElifStmt {
            expr: ASTExpr::Number(1),
            block: vec![ASTStatement::Expr(ASTExpr::Number(2))],
        });
        let result = parse_elif_stmt(&mut parser);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_else_stmt() {
        let mut parser = create_parser("else:\n    2");
        let expected = ASTStatement::Else(ASTElseStmt {
            block: vec![ASTStatement::Expr(ASTExpr::Number(2))],
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
                identifier: "i",
                infer_type: true,
                typehint: None,
                value: None,
            },
            container: ASTExpr::Array(vec![ASTExpr::Number(1), ASTExpr::Number(2)]),
            block: vec![ASTStatement::Expr(ASTExpr::Number(3))],
        });
        let result = parse_for_stmt(&mut parser);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_if_stmt() {
        let mut parser = create_parser("if 1:\n    2");
        let expected = ASTStatement::If(ASTIfStmt {
            expr: ASTExpr::Number(1),
            block: vec![ASTStatement::Expr(ASTExpr::Number(2))],
        });
        let result = parse_if_stmt(&mut parser);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_while_stmt() {
        let mut parser = create_parser("while 1:\n    2");
        let expected = ASTStatement::While(ASTWhileStmt {
            expr: ASTExpr::Number(1),
            block: vec![ASTStatement::Expr(ASTExpr::Number(2))],
        });
        let result = parse_while_stmt(&mut parser);

        assert_eq!(result, expected);
    }
}
