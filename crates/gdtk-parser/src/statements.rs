use std::iter::Peekable;

use gdtk_ast::poor::{
    ASTElifStmt, ASTElseStmt, ASTForStmt, ASTIfStmt, ASTStatement, ASTValue, ASTVariable,
    ASTVariableKind, ASTWhileStmt, CodeBlock,
};
use gdtk_lexer::{Token, TokenKind};

use crate::{
    block::parse_block,
    expressions::parse_expr,
    utils::{expect, peek_non_blank},
    variables::parse_variable_body,
};

/// Parses a `return` statement.
pub fn parse_return_stmt<'a>(
    iter: &mut Peekable<impl Iterator<Item = Token<'a>>>,
) -> ASTStatement<'a> {
    iter.next();
    ASTStatement::Return(parse_expr(iter))
}

/// Parses a `for` statement.
pub fn parse_for_stmt<'a>(
    iter: &mut Peekable<impl Iterator<Item = Token<'a>>>,
) -> ASTStatement<'a> {
    expect!(iter, TokenKind::For);
    let identifier = expect!(iter, TokenKind::Identifier(s), s);
    let mut typehint = None;

    if peek_non_blank(iter).is_some_and(|t| t.kind.is_colon()) {
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
    iter.next();
    ASTStatement::Variable(parse_variable_body(iter, ASTVariableKind::Regular))
}

/// Parses a `const` statement.
pub fn parse_const_stmt<'a>(
    iter: &mut Peekable<impl Iterator<Item = Token<'a>>>,
) -> ASTStatement<'a> {
    iter.next();
    ASTStatement::Variable(parse_variable_body(iter, ASTVariableKind::Constant))
}

/// Parses a `static var` statement.
pub fn parse_static_var_stmt<'a>(
    iter: &mut Peekable<impl Iterator<Item = Token<'a>>>,
) -> ASTStatement<'a> {
    iter.next();
    expect!(iter, TokenKind::Var);
    ASTStatement::Variable(parse_variable_body(iter, ASTVariableKind::Static))
}

/// Parses an `else` statement.
pub fn parse_else_stmt<'a>(
    iter: &mut Peekable<impl Iterator<Item = Token<'a>>>,
) -> ASTStatement<'a> {
    iter.next();
    expect!(iter, TokenKind::Colon);
    ASTStatement::Else(ASTElseStmt {
        block: parse_block(iter, false),
    })
}

/// Parses an `if` statement.
pub fn parse_if_stmt<'a>(iter: &mut Peekable<impl Iterator<Item = Token<'a>>>) -> ASTStatement<'a> {
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
    let tuple = parse_iflike(iter);
    ASTStatement::While(ASTWhileStmt {
        expr: tuple.0,
        block: tuple.1,
    })
}

fn parse_iflike<'a>(
    iter: &mut Peekable<impl Iterator<Item = Token<'a>>>,
) -> (ASTValue<'a>, CodeBlock<'a>) {
    expect!(iter, TokenKind::If | TokenKind::Elif | TokenKind::While);
    let cond = parse_expr(iter);
    expect!(iter, TokenKind::Colon);
    let block = parse_block(iter, false);

    (cond, block)
}
