use gdtk_ast::ASTStatement;
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
use crate::Parser;

pub fn parse_statement<'a>(
    parser: &mut Parser<impl Iterator<Item = Token<'a>>>,
) -> ASTStatement<'a> {
    match parser.peek().expect("expected a statement, found EOF").kind {
        TokenKind::Annotation => ASTStatement::Annotation(parse_annotation(parser)),
        TokenKind::Assert => ASTStatement::Assert(advance_and_parse(parser, parse_expr)),
        TokenKind::Break => advance_and_parse(parser, |_| ASTStatement::Break),
        TokenKind::Breakpoint => advance_and_parse(parser, |_| ASTStatement::Breakpoint),
        TokenKind::Class => ASTStatement::Class(parse_class(parser)),
        TokenKind::ClassName => parse_classname_stmt(parser),
        TokenKind::Continue => advance_and_parse(parser, |_| ASTStatement::Continue),
        TokenKind::If => parse_if_stmt(parser),
        TokenKind::Elif => parse_elif_stmt(parser),
        TokenKind::Else => parse_else_stmt(parser),
        TokenKind::Enum => ASTStatement::Enum(parse_enum(parser)),
        TokenKind::Extends => parse_extends_stmt(parser),
        TokenKind::For => parse_for_stmt(parser),
        TokenKind::Pass => advance_and_parse(parser, |_| ASTStatement::Pass),
        TokenKind::Func => ASTStatement::Func(parse_func(parser, false)),
        TokenKind::Return => parse_return_stmt(parser),
        TokenKind::Signal => ASTStatement::Signal(parse_signal(parser)),
        TokenKind::Match => ASTStatement::Match(parse_match(parser)),
        TokenKind::While => parse_while_stmt(parser),
        TokenKind::Var => parse_var_stmt(parser),
        TokenKind::Const => parse_const_stmt(parser),
        TokenKind::Static => parse_static_var_stmt(parser),
        _ => ASTStatement::Value(parse_expr(parser)),
    }
}

// Do we really need tests for `parse_statement`?
#[cfg(test)]
mod tests {}
