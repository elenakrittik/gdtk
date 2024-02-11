use std::iter::Peekable;

use gdtk_ast::poor::{ASTValue, ASTStatement, CodeBlock, ASTAssignmentKind};
use gdtk_lexer::{Token, TokenKind};

use crate::block::parse_block;
use crate::values::parse_value;
use crate::variables::{parse_var, parse_const};
use crate::utils::{peek_non_blank, next_non_blank, expect_blank_prefixed, any_assignment};

pub fn parse_statement<'a, T>(iter: &mut Peekable<T>, mut token: Option<Token<'a>>) -> ASTStatement<'a>
where
    T: Iterator<Item = Token<'a>>,
{
    if token.is_none() {
        token.replace(next_non_blank!(iter));
    }

    let token = token.unwrap();

    match token.kind {
        TokenKind::Assert => ASTStatement::Assert(parse_value(iter, None)),
        TokenKind::Identifier(s) => {
            if matches!(peek_non_blank!(iter).kind, any_assignment!(TokenKind)) {
                parse_assignment(iter, s)
            } else {
                ASTStatement::Value(parse_value(iter, Some(token)))
            }
        },
        TokenKind::Break => ASTStatement::Break,
        TokenKind::Breakpoint => ASTStatement::Breakpoint,
        TokenKind::Continue => ASTStatement::Continue,
        TokenKind::If => {
            let tuple = parse_iflike(iter);
            ASTStatement::If(tuple.0, tuple.1)
        },
        TokenKind::Elif => {
            let tuple = parse_iflike(iter);
            ASTStatement::Elif(tuple.0, tuple.1)
        },
        TokenKind::Else => {
            expect_blank_prefixed!(iter, TokenKind::Colon, ());
            ASTStatement::Else(parse_block(iter))
        },
        TokenKind::For => parse_for_loop(iter),
        TokenKind::Pass => ASTStatement::Pass,
        TokenKind::Return => ASTStatement::Return(parse_value(iter, None)),
        TokenKind::Match => todo!(),
        TokenKind::While => parse_while_loop(iter),
        TokenKind::Var => ASTStatement::Variable(parse_var(iter)),
        TokenKind::Const => ASTStatement::Variable(parse_const(iter)),
        TokenKind::Static => todo!(),
        _ => ASTStatement::Value(parse_value(iter, Some(token))),
    }
}

pub fn parse_iflike<'a, T>(iter: &mut Peekable<T>) -> (ASTValue<'a>, CodeBlock<'a>)
where
    T: Iterator<Item = Token<'a>>,
{
    let cond = parse_value(iter, None);
    expect_blank_prefixed!(iter, TokenKind::Colon, ());
    let code = parse_block(iter);

    (cond, code)
}

pub fn parse_for_loop<'a, T>(iter: &mut Peekable<T>) -> ASTStatement<'a>
where
    T: Iterator<Item = Token<'a>>,
{
    let identifier = expect_blank_prefixed!(iter, TokenKind::Identifier(s), s);
    let mut type_hint = None;

    match peek_non_blank!(iter) {
        Token { kind: TokenKind::Colon, .. } => {
            expect_blank_prefixed!(iter, TokenKind::Colon, ());
            type_hint = Some(parse_value(iter, None));
        },
        _ => ()
    }

    expect_blank_prefixed!(iter, TokenKind::In, ());
    let container = parse_value(iter, None);
    expect_blank_prefixed!(iter, TokenKind::Colon, ());
    let block = parse_block(iter);

    ASTStatement::For(identifier, type_hint, container, block)
}

pub fn parse_while_loop<'a, T>(iter: &mut Peekable<T>) -> ASTStatement<'a>
where
    T: Iterator<Item = Token<'a>>,
{
    let expr = parse_value(iter, None);
    expect_blank_prefixed!(iter, TokenKind::Colon, ());
    let block = parse_block(iter);

    ASTStatement::While(expr, block)
}

pub fn parse_assignment<'a, T>(iter: &mut Peekable<T>, identifier: &'a str) -> ASTStatement<'a>
where
    T: Iterator<Item = Token<'a>>,
{
    let kind = match next_non_blank!(iter).kind {
        TokenKind::Assignment => ASTAssignmentKind::Regular,
        TokenKind::PlusAssignment => ASTAssignmentKind::Plus,
        TokenKind::MinusAssignment => ASTAssignmentKind::Minus,
        TokenKind::MultiplyAssignment => ASTAssignmentKind::Multiply,
        TokenKind::PowerAssignment => ASTAssignmentKind::Power,
        TokenKind::DivideAssignment => ASTAssignmentKind::Divide,
        TokenKind::RemainderAssignment => ASTAssignmentKind::Remainder,
        TokenKind::BitwiseAndAssignment => ASTAssignmentKind::BitwiseAnd,
        TokenKind::BitwiseOrAssignment => ASTAssignmentKind::BitwiseOr,
        TokenKind::BitwiseNotAssignment => ASTAssignmentKind::BitwiseNot,
        TokenKind::BitwiseXorAssignment => ASTAssignmentKind::BitwiseXor,
        TokenKind::BitwiseShiftLeftAssignment => ASTAssignmentKind::BitwiseShiftLeft,
        TokenKind::BitwiseShiftRightAssignment => ASTAssignmentKind::BitwiseShiftRight,
        _ => panic!("impossibal!!11!1"),
    };

    let value = parse_value(iter, None);

    ASTStatement::Assignment(identifier, kind, value)
}
