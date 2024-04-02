use std::iter::Peekable;

use gdtk_ast::poor::{
    ASTStatement, ASTValue, ASTVariable, ASTVariableKind,
    CodeBlock,
};
use gdtk_lexer::{Token, TokenKind};

use crate::block::parse_block;
use crate::classes::{parse_class, parse_classname, parse_enum, parse_extends};
use crate::expressions::parse_expr;
use crate::functions::parse_func;
use crate::match_::parse_match;
use crate::misc::{parse_annotation, parse_signal};
use crate::utils::{advance_and_parse, expect_blank_prefixed, peek_non_blank};
use crate::variables::parse_variable_body;

pub fn parse_statement<'a>(
    iter: &mut Peekable<impl Iterator<Item = Token<'a>>>,
) -> ASTStatement<'a> {
    match peek_non_blank(iter)
        .expect("expected a statement, found EOF")
        .kind
    {
        TokenKind::Annotation => parse_annotation(iter),
        TokenKind::Assert => ASTStatement::Assert(advance_and_parse(iter, parse_expr)),
        TokenKind::Break => advance_and_parse(iter, |_| ASTStatement::Break),
        TokenKind::Breakpoint => advance_and_parse(iter, |_| ASTStatement::Breakpoint),
        TokenKind::Class => ASTStatement::Class(parse_class(iter)),
        TokenKind::ClassName => parse_classname(iter),
        TokenKind::Continue => advance_and_parse(iter, |_| ASTStatement::Continue),
        TokenKind::If => {
            let tuple = parse_iflike(iter);
            ASTStatement::If(tuple.0, tuple.1)
        }
        TokenKind::Elif => {
            let tuple = parse_iflike(iter);
            ASTStatement::Elif(tuple.0, tuple.1)
        }
        TokenKind::Else => {
            iter.next();
            expect_blank_prefixed!(iter, TokenKind::Colon, ());
            ASTStatement::Else(parse_block(iter, false))
        }
        TokenKind::Enum => parse_enum(iter),
        TokenKind::Extends => parse_extends(iter),
        TokenKind::For => parse_for_loop(iter),
        TokenKind::Pass => advance_and_parse(iter, |_| ASTStatement::Pass),
        TokenKind::Func => ASTStatement::Func(parse_func(iter, false)),
        TokenKind::Return => advance_and_parse(iter, |iter| ASTStatement::Return(parse_expr(iter))),
        TokenKind::Signal => ASTStatement::Signal(parse_signal(iter)),
        TokenKind::Match => parse_match(iter),
        TokenKind::While => {
            let tuple = parse_iflike(iter);
            ASTStatement::While(tuple.0, tuple.1)
        }
        TokenKind::Var => advance_and_parse(iter, |iter| {
            ASTStatement::Variable(parse_variable_body(iter, ASTVariableKind::Regular))
        }),
        TokenKind::Const => advance_and_parse(iter, |iter| {
            ASTStatement::Variable(parse_variable_body(iter, ASTVariableKind::Constant))
        }),
        TokenKind::Static => {
            iter.next();
            expect_blank_prefixed!(iter, TokenKind::Var, ());
            ASTStatement::Variable(parse_variable_body(iter, ASTVariableKind::Static))
        }
        _ => ASTStatement::Value(parse_expr(iter)),
    }
}

pub fn parse_iflike<'a>(
    iter: &mut Peekable<impl Iterator<Item = Token<'a>>>,
) -> (ASTValue<'a>, CodeBlock<'a>) {
    expect_blank_prefixed!(iter, TokenKind::If | TokenKind::Elif | TokenKind::While, ());
    let cond = parse_expr(iter);
    expect_blank_prefixed!(iter, TokenKind::Colon, ());
    let block = parse_block(iter, false);

    (cond, block)
}

pub fn parse_for_loop<'a>(
    iter: &mut Peekable<impl Iterator<Item = Token<'a>>>,
) -> ASTStatement<'a> {
    expect_blank_prefixed!(iter, TokenKind::For, ());
    let identifier = expect_blank_prefixed!(iter, TokenKind::Identifier(s), s);
    let mut type_hint = None;

    if peek_non_blank(iter).is_some_and(|t| t.kind.is_colon()) {
        iter.next();
        type_hint = Some(parse_expr(iter));
    }

    expect_blank_prefixed!(iter, TokenKind::In, ());
    let container = parse_expr(iter);
    expect_blank_prefixed!(iter, TokenKind::Colon, ());
    let block = parse_block(iter, false);

    ASTStatement::For(identifier, type_hint, container, block)
}
