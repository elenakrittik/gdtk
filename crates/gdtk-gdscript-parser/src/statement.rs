use gdtk_ast::{ASTFunctionKind, ASTStatement};
use crate::lexer::{Token, TokenKind};

use crate::classes::{parse_class, parse_enum};
use crate::expressions::parse_expr;
use crate::functions::{parse_func, ParseFuncOptions};
use crate::match_::parse_match;
use crate::misc::{parse_annotation, parse_signal};
use crate::statements::{
    parse_assert_stmt, parse_break_stmt, parse_breakpoint_stmt, parse_classname_stmt,
    parse_const_stmt, parse_continue_stmt, parse_elif_stmt, parse_else_stmt, parse_extends_stmt,
    parse_for_stmt, parse_if_stmt, parse_pass_stmt, parse_return_stmt, parse_static_var_stmt,
    parse_var_stmt, parse_while_stmt,
};
use crate::Parser;

pub fn parse_statement<'a>(
    parser: &mut Parser<'a, impl Iterator<Item = Token<'a>>>,
) -> ASTStatement<'a> {
    match parser.peek().expect("expected a statement, found EOF").kind {
        TokenKind::Annotation => ASTStatement::Annotation(parse_annotation(parser)),
        TokenKind::Assert => parse_assert_stmt(parser),
        TokenKind::Break => parse_break_stmt(parser),
        TokenKind::Breakpoint => parse_breakpoint_stmt(parser),
        TokenKind::Class => ASTStatement::Class(parse_class(parser)),
        TokenKind::ClassName => parse_classname_stmt(parser),
        TokenKind::Continue => parse_continue_stmt(parser),
        TokenKind::If => parse_if_stmt(parser),
        TokenKind::Elif => parse_elif_stmt(parser),
        TokenKind::Else => parse_else_stmt(parser),
        TokenKind::Enum => ASTStatement::Enum(parse_enum(parser)),
        TokenKind::Extends => parse_extends_stmt(parser),
        TokenKind::For => parse_for_stmt(parser),
        TokenKind::Pass => parse_pass_stmt(parser),
        TokenKind::Func => ASTStatement::Func(parse_func(
            parser,
            ParseFuncOptions {
                kind: ASTFunctionKind::Regular,
                is_lambda: false,
            },
        )),
        TokenKind::Return => parse_return_stmt(parser),
        TokenKind::Signal => ASTStatement::Signal(parse_signal(parser)),
        TokenKind::Match => ASTStatement::Match(parse_match(parser)),
        TokenKind::While => parse_while_stmt(parser),
        TokenKind::Var => parse_var_stmt(parser),
        TokenKind::Const => parse_const_stmt(parser),
        TokenKind::Static => {
            parser.next(); // we have to consume it. sorry

            match parser
                .peek()
                .expect("expected `var` or `func`, found EOF")
                .kind
            {
                TokenKind::Var => parse_static_var_stmt(parser),
                TokenKind::Func => ASTStatement::Func(parse_func(
                    parser,
                    ParseFuncOptions {
                        kind: ASTFunctionKind::Static,
                        is_lambda: false,
                    },
                )),
                _ => panic!("expected `var` or `func`, found `{:?}`", parser.peek()),
            }
        }
        _ => ASTStatement::Expr(parse_expr(parser)),
    }
}

// Do we really need tests for `parse_statement`?
#[cfg(test)]
mod tests {}
