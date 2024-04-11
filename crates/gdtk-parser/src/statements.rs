use std::iter::Peekable;

use gdtk_ast::poor::{
    ASTElifStmt, ASTElseStmt, ASTForStmt, ASTIfStmt, ASTStatement, ASTValue, ASTVariable,
    ASTVariableKind, ASTWhileStmt, CodeBlock,
};
use gdtk_lexer::{Token, TokenKind};

use crate::{
    block::parse_block, expressions::parse_expr, utils::expect, variables::parse_variable_body,
};

/// Parses a `return` statement.
pub fn parse_return_stmt<'a>(
    iter: &mut Peekable<impl Iterator<Item = Token<'a>>>,
) -> ASTStatement<'a> {
    expect!(iter, TokenKind::Return);

    let value = if iter.peek().is_some_and(|t| !{
        t.kind.is_newline()
            || t.kind.is_closing_brace()
            || t.kind.is_closing_bracket()
            || t.kind.is_closing_parenthesis()
            || t.kind.is_semicolon()
    }) {
        Some(parse_expr(iter))
    } else {
        None
    };

    ASTStatement::Return(value)
}

/// Parses a `for` statement.
pub fn parse_for_stmt<'a>(
    iter: &mut Peekable<impl Iterator<Item = Token<'a>>>,
) -> ASTStatement<'a> {
    expect!(iter, TokenKind::For);
    let identifier = expect!(iter, TokenKind::Identifier(s), s);
    let mut typehint = None;

    if iter.peek().is_some_and(|t| t.kind.is_colon()) {
        iter.next();
        typehint = Some(parse_expr(iter));
    }

    expect!(iter, TokenKind::In);
    let container = parse_expr(iter);
    expect!(iter, TokenKind::Colon);
    let block = parse_block(iter, false);

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
    iter: &mut Peekable<impl Iterator<Item = Token<'a>>>,
) -> ASTStatement<'a> {
    expect!(iter, TokenKind::ClassName);
    expect!(iter, TokenKind::Identifier(i), ASTStatement::ClassName(i))
}

/// Parses an `extends` statement.
pub fn parse_extends_stmt<'a>(
    iter: &mut Peekable<impl Iterator<Item = Token<'a>>>,
) -> ASTStatement<'a> {
    expect!(iter, TokenKind::Extends);
    expect!(iter, TokenKind::Identifier(i), ASTStatement::Extends(i))
}

/// Parses a `var` statement.
pub fn parse_var_stmt<'a>(
    iter: &mut Peekable<impl Iterator<Item = Token<'a>>>,
) -> ASTStatement<'a> {
    expect!(iter, TokenKind::Var);
    ASTStatement::Variable(parse_variable_body(iter, ASTVariableKind::Regular))
}

/// Parses a `const` statement.
pub fn parse_const_stmt<'a>(
    iter: &mut Peekable<impl Iterator<Item = Token<'a>>>,
) -> ASTStatement<'a> {
    expect!(iter, TokenKind::Const);
    ASTStatement::Variable(parse_variable_body(iter, ASTVariableKind::Constant))
}

/// Parses a `static var` statement.
pub fn parse_static_var_stmt<'a>(
    iter: &mut Peekable<impl Iterator<Item = Token<'a>>>,
) -> ASTStatement<'a> {
    expect!(iter, TokenKind::Static);
    expect!(iter, TokenKind::Var);
    ASTStatement::Variable(parse_variable_body(iter, ASTVariableKind::Static))
}

/// Parses an `else` statement.
pub fn parse_else_stmt<'a>(
    iter: &mut Peekable<impl Iterator<Item = Token<'a>>>,
) -> ASTStatement<'a> {
    expect!(iter, TokenKind::Else);
    expect!(iter, TokenKind::Colon);
    ASTStatement::Else(ASTElseStmt {
        block: parse_block(iter, false),
    })
}

/// Parses an `if` statement.
pub fn parse_if_stmt<'a>(iter: &mut Peekable<impl Iterator<Item = Token<'a>>>) -> ASTStatement<'a> {
    expect!(iter, TokenKind::If);
    let tuple = parse_iflike(iter);
    ASTStatement::If(ASTIfStmt {
        expr: tuple.0,
        block: tuple.1,
    })
}

/// Parses an `elif` statement.
pub fn parse_elif_stmt<'a>(
    iter: &mut Peekable<impl Iterator<Item = Token<'a>>>,
) -> ASTStatement<'a> {
    expect!(iter, TokenKind::Elif);
    let tuple = parse_iflike(iter);
    ASTStatement::Elif(ASTElifStmt {
        expr: tuple.0,
        block: tuple.1,
    })
}

/// Parses a `while` statement.
pub fn parse_while_stmt<'a>(
    iter: &mut Peekable<impl Iterator<Item = Token<'a>>>,
) -> ASTStatement<'a> {
    expect!(iter, TokenKind::While);
    let tuple = parse_iflike(iter);
    ASTStatement::While(ASTWhileStmt {
        expr: tuple.0,
        block: tuple.1,
    })
}

/// Parse a
fn parse_iflike<'a>(
    iter: &mut Peekable<impl Iterator<Item = Token<'a>>>,
) -> (ASTValue<'a>, CodeBlock<'a>) {
    let cond = parse_expr(iter);
    expect!(iter, TokenKind::Colon);
    let block = parse_block(iter, false);

    (cond, block)
}

#[cfg(test)]
mod tests {
    use gdtk_ast::poor::*;

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
            value: Some(ASTValue::Number(1)),
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
            value: Some(ASTValue::Number(1)),
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
            value: Some(ASTValue::Number(1)),
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
        let expected = ASTStatement::Return(Some(ASTValue::Number(1)));
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
            expr: ASTValue::Number(1),
            block: vec![ASTStatement::Value(ASTValue::Number(2))],
        });
        let result = parse_elif_stmt(&mut parser);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_else_stmt() {
        let mut parser = create_parser("else:\n    2");
        let expected = ASTStatement::Else(ASTElseStmt {
            block: vec![ASTStatement::Value(ASTValue::Number(2))],
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
            container: ASTValue::Array(vec![ASTValue::Number(1), ASTValue::Number(2)]),
            block: vec![ASTStatement::Value(ASTValue::Number(3))],
        });
        let result = parse_for_stmt(&mut parser);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_if_stmt() {
        let mut parser = create_parser("if 1:\n    2");
        let expected = ASTStatement::If(ASTIfStmt {
            expr: ASTValue::Number(1),
            block: vec![ASTStatement::Value(ASTValue::Number(2))],
        });
        let result = parse_if_stmt(&mut parser);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_while_stmt() {
        let mut parser = create_parser("while 1:\n    2");
        let expected = ASTStatement::While(ASTWhileStmt {
            expr: ASTValue::Number(1),
            block: vec![ASTStatement::Value(ASTValue::Number(2))],
        });
        let result = parse_while_stmt(&mut parser);

        assert_eq!(result, expected);
    }
}
