use std::iter::Peekable;

use gdtk_ast::poor::{
    ASTAssignmentKind, ASTMatchPattern, ASTMatchPatternKind, ASTStatement, ASTValue, ASTVariable,
    ASTVariableKind, CodeBlock,
};
use gdtk_lexer::{Token, TokenKind};

use crate::block::parse_block;
use crate::classes::{parse_class, parse_classname, parse_enum, parse_extends};
use crate::functions::parse_func;
use crate::misc::{parse_annotation, parse_signal};
use crate::utils::{expect, expect_blank_prefixed, next_non_blank, peek_non_blank};
use crate::expressions::parse_expression;
use crate::variables::parse_variable;

pub fn parse_statement<'a, T>(
    iter: &mut Peekable<T>,
    mut token: Option<Token<'a>>,
) -> ASTStatement<'a>
where
    T: Iterator<Item = Token<'a>>,
{
    if token.is_none() {
        token.replace(next_non_blank!(iter));
    }

    let token = token.unwrap();

    match token.kind {
        TokenKind::Annotation => parse_annotation(iter),
        TokenKind::Assert => ASTStatement::Assert(parse_expression(iter)),
        TokenKind::Identifier(s) => {
            if peek_non_blank(iter).is_some_and(|t| t.kind.is_any_assignment()) {
                parse_assignment(iter, s)
            } else {
                ASTStatement::Value(parse_value(iter, Some(token)))
            }
        }
        TokenKind::Break => ASTStatement::Break,
        TokenKind::Breakpoint => ASTStatement::Breakpoint,
        TokenKind::Class => ASTStatement::Class(parse_class(iter)),
        TokenKind::ClassName => parse_classname(iter),
        TokenKind::Continue => ASTStatement::Continue,
        TokenKind::If => {
            let tuple = parse_iflike(iter);
            ASTStatement::If(tuple.0, tuple.1)
        }
        TokenKind::Elif => {
            let tuple = parse_iflike(iter);
            ASTStatement::Elif(tuple.0, tuple.1)
        }
        TokenKind::Else => {
            expect_blank_prefixed!(iter, TokenKind::Colon, ());
            ASTStatement::Else(parse_block(iter, false))
        }
        TokenKind::Enum => parse_enum(iter),
        TokenKind::Extends => parse_extends(iter),
        TokenKind::For => parse_for_loop(iter),
        TokenKind::Pass => ASTStatement::Pass,
        TokenKind::Func => ASTStatement::Func(parse_func(iter, false)),
        TokenKind::Return => ASTStatement::Return(parse_expression(iter)),
        TokenKind::Signal => ASTStatement::Signal(parse_signal(iter)),
        TokenKind::Match => parse_match(iter),
        TokenKind::While => {
            let tuple = parse_iflike(iter);
            ASTStatement::While(tuple.0, tuple.1)
        }
        TokenKind::Var => ASTStatement::Variable(parse_variable(iter, ASTVariableKind::Regular)),
        TokenKind::Const => ASTStatement::Variable(parse_variable(iter, ASTVariableKind::Constant)),
        TokenKind::Static => {
            expect_blank_prefixed!(iter, TokenKind::Var, ());
            ASTStatement::Variable(parse_variable(iter, ASTVariableKind::Static))
        }
        _ => ASTStatement::Value(parse_value(iter, Some(token))),
    }
}

pub fn parse_iflike<'a, T>(iter: &mut Peekable<T>) -> (ASTValue<'a>, CodeBlock<'a>)
where
    T: Iterator<Item = Token<'a>>,
{
    let cond = parse_expression(iter);
    expect_blank_prefixed!(iter, TokenKind::Colon, ());
    let code = parse_block(iter, false);

    (cond, code)
}

pub fn parse_for_loop<'a, T>(iter: &mut Peekable<T>) -> ASTStatement<'a>
where
    T: Iterator<Item = Token<'a>>,
{
    let identifier = expect_blank_prefixed!(iter, TokenKind::Identifier(s), s);
    let mut type_hint = None;

    if peek_non_blank(iter).is_some_and(|t| t.kind.is_colon()) {
        iter.next();
        type_hint = Some(parse_expression(iter));
    }

    expect_blank_prefixed!(iter, TokenKind::In, ());
    let container = parse_expression(iter);
    expect_blank_prefixed!(iter, TokenKind::Colon, ());
    let block = parse_block(iter, false);

    ASTStatement::For(identifier, type_hint, container, block)
}

pub fn parse_while_loop<'a, T>(iter: &mut Peekable<T>) -> ASTStatement<'a>
where
    T: Iterator<Item = Token<'a>>,
{
    let expr = parse_expression(iter);
    expect_blank_prefixed!(iter, TokenKind::Colon, ());
    let block = parse_block(iter, false);

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
        other => panic!("impossible {other:?}"),
    };

    let value = parse_expression(iter);

    ASTStatement::Assignment(identifier, kind, value)
}

pub fn parse_match<'a, T>(iter: &mut Peekable<T>) -> ASTStatement<'a>
where
    T: Iterator<Item = Token<'a>>,
{
    let expr = parse_expression(iter);
    let mut pats = vec![];

    expect_blank_prefixed!(iter, TokenKind::Colon, ());
    expect_blank_prefixed!(iter, TokenKind::Newline, ());
    expect!(iter, TokenKind::Indent, ());

    loop {
        match peek_non_blank(iter).expect("unexpected EOF") {
            Token {
                kind: TokenKind::Dedent,
                ..
            } => {
                iter.next();
                break;
            }
            Token {
                kind: TokenKind::Newline,
                ..
            } => continue,
            _ => (),
        };

        let pat = parse_pat(iter);
        expect_blank_prefixed!(iter, TokenKind::Colon, ());
        let block = parse_block(iter, false);

        pats.push(ASTMatchPattern {
            body: block,
            kind: pat,
        });
    }

    ASTStatement::Match(expr, pats)
}

pub fn parse_pat<'a, T>(iter: &mut Peekable<T>) -> ASTMatchPatternKind<'a>
where
    T: Iterator<Item = Token<'a>>,
{
    let temp = match peek_non_blank(iter).expect("unexpected EOF").kind {
        TokenKind::OpeningBrace => todo!(),
        TokenKind::Var => {
            iter.next();

            let identifier = expect_blank_prefixed!(iter, TokenKind::Identifier(s), s);
            ASTMatchPatternKind::Binding(ASTVariable {
                identifier,
                infer_type: false,
                typehint: None,
                value: None,
                kind: ASTVariableKind::Regular,
            })
        }
        TokenKind::OpeningBracket => {
            iter.next();

            let mut pats = vec![];
            let mut expect_pat = true;

            loop {
                match peek_non_blank(iter).expect("unexpected eof") {
                    Token {
                        kind: TokenKind::ClosingBracket,
                        ..
                    } => {
                        iter.next();
                        break;
                    }
                    other => {
                        if !expect_pat {
                            panic!("unexpected {other:?}");
                        }

                        pats.push(parse_pat(iter));

                        if !peek_non_blank(iter).is_some_and(|t| t.kind.is_comma()) {
                            expect_pat = false;
                        } else {
                            iter.next();
                        }
                    }
                }
            }

            ASTMatchPatternKind::Array(pats)
        }
        TokenKind::Range => {
            iter.next();
            ASTMatchPatternKind::Rest
        }
        _ => {
            ASTMatchPatternKind::Value(parse_expression(iter))
        }
    };

    temp
}
