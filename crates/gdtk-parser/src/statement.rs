use std::iter::Peekable;

use gdtk_ast::poor::ASTStatement;
use gdtk_lexer::{Token, TokenKind};

use crate::classes::{parse_class, parse_enum};
use crate::expressions::parse_expr;
use crate::functions::parse_func;
use crate::match_::parse_match;
use crate::misc::{parse_annotation, parse_signal};
use crate::statements::{
    parse_classname_stmt, parse_const_stmt, parse_elif_stmt, parse_else_stmt, parse_extends_stmt,
    parse_for_stmt, parse_if_stmt, parse_return_stmt, parse_static_var_stmt, parse_var_stmt,
    parse_while_stmt,
};
use crate::utils::advance_and_parse;

pub fn parse_statement<'a>(
    iter: &mut Peekable<impl Iterator<Item = Token<'a>>>,
) -> ASTStatement<'a> {
    match iter.peek().expect("expected a statement, found EOF").kind {
        TokenKind::Annotation => ASTStatement::Annotation(parse_annotation(iter)),
        TokenKind::Assert => ASTStatement::Assert(advance_and_parse(iter, parse_expr)),
        TokenKind::Break => advance_and_parse(iter, |_| ASTStatement::Break),
        TokenKind::Breakpoint => advance_and_parse(iter, |_| ASTStatement::Breakpoint),
        TokenKind::Class => ASTStatement::Class(parse_class(iter)),
        TokenKind::ClassName => parse_classname_stmt(iter),
        TokenKind::Continue => advance_and_parse(iter, |_| ASTStatement::Continue),
        TokenKind::If => parse_if_stmt(iter),
        TokenKind::Elif => parse_elif_stmt(iter),
        TokenKind::Else => parse_else_stmt(iter),
        TokenKind::Enum => ASTStatement::Enum(parse_enum(iter)),
        TokenKind::Extends => parse_extends_stmt(iter),
        TokenKind::For => parse_for_stmt(iter),
        TokenKind::Pass => advance_and_parse(iter, |_| ASTStatement::Pass),
        TokenKind::Func => ASTStatement::Func(parse_func(iter, false)),
        TokenKind::Return => parse_return_stmt(iter),
        TokenKind::Signal => ASTStatement::Signal(parse_signal(iter)),
        TokenKind::Match => ASTStatement::Match(parse_match(iter)),
        TokenKind::While => parse_while_stmt(iter),
        TokenKind::Var => parse_var_stmt(iter),
        TokenKind::Const => parse_const_stmt(iter),
        TokenKind::Static => parse_static_var_stmt(iter),
        _ => ASTStatement::Value(parse_expr(iter)),
    }
}
