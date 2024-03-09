use std::iter::Peekable;

use gdtk_ast::poor::{
    ASTAssignmentKind, ASTMatchPattern, ASTMatchPatternKind, ASTStatement, ASTValue, ASTVariable,
    ASTVariableKind, CodeBlock,
};
use gdtk_lexer::{Token, TokenKind};

use crate::block::parse_block;
use crate::classes::{parse_classname, parse_enum, parse_extends};
use crate::functions::parse_func;
use crate::misc::parse_annotation;
use crate::utils::{any_assignment, expect, expect_blank_prefixed, next_non_blank, peek_non_blank};
use crate::values::parse_value;
use crate::variables::{parse_const, parse_var};

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
        TokenKind::Assert => ASTStatement::Assert(parse_value(iter, None)),
        TokenKind::Identifier(s) => {
            if matches!(peek_non_blank!(iter).kind, any_assignment!(TokenKind)) {
                parse_assignment(iter, s)
            } else {
                ASTStatement::Value(parse_value(iter, Some(token)))
            }
        }
        TokenKind::Break => ASTStatement::Break,
        TokenKind::Breakpoint => ASTStatement::Breakpoint,
        TokenKind::Class => todo!(),
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
            ASTStatement::Else(parse_block(iter))
        }
        TokenKind::Enum => parse_enum(iter),
        TokenKind::Extends => parse_extends(iter),
        TokenKind::For => parse_for_loop(iter),
        TokenKind::Pass => ASTStatement::Pass,
        TokenKind::Func => parse_func(iter),
        TokenKind::Return => ASTStatement::Return(parse_value(iter, None)),
        TokenKind::Signal => todo!(),
        TokenKind::Match => parse_match(iter),
        TokenKind::While => {
            let tuple = parse_iflike(iter);
            ASTStatement::While(tuple.0, tuple.1)
        }
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

    if let TokenKind::Colon = peek_non_blank!(iter).kind {
        iter.next();
        type_hint = Some(parse_value(iter, None));
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
        other => panic!("impossible {other:?}"),
    };

    let value = parse_value(iter, None);

    ASTStatement::Assignment(identifier, kind, value)
}

pub fn parse_match<'a, T>(iter: &mut Peekable<T>) -> ASTStatement<'a>
where
    T: Iterator<Item = Token<'a>>,
{
    let expr = parse_value(iter, None);
    eprintln!("parsing match with expr = {:?}", &expr);
    let mut pats = vec![];
    expect_blank_prefixed!(iter, TokenKind::Colon, ());
    expect_blank_prefixed!(iter, TokenKind::Newline, ());
    expect!(iter, TokenKind::Indent, ());

    loop {
        match peek_non_blank!(iter) {
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
        let block = parse_block(iter);

        pats.push(ASTMatchPattern {
            body: block,
            kind: pat,
        });
    }

    eprintln!("end parse match with pats = {:?}", &pats);

    ASTStatement::Match(expr, pats)
}

pub fn parse_pat<'a, T>(iter: &mut Peekable<T>) -> ASTMatchPatternKind<'a>
where
    T: Iterator<Item = Token<'a>>,
{
    eprintln!("parsing pattern");
    let temp = match peek_non_blank!(iter).kind {
        TokenKind::OpeningBrace => todo!(),
        TokenKind::Var => {
            eprintln!("parsing pat var");
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
            eprintln!("parsing pat array");
            iter.next();

            let mut pats = vec![];
            let mut expect_pat = true;

            loop {
                eprintln!("parsing array pat; expect pat is {expect_pat}");
                match peek_non_blank!(iter) {
                    Token {
                        kind: TokenKind::ClosingBracket,
                        ..
                    } => {
                        eprintln!("got closing bracket, breaking");
                        iter.next();
                        break;
                    }
                    other => {
                        eprintln!("got {:?}", &other);
                        if !expect_pat {
                            panic!("unexpected {other:?}");
                        }

                        pats.push(parse_pat(iter));

                        eprintln!("next token: {:?}", &peek_non_blank!(iter));

                        if !matches!(peek_non_blank!(iter).kind, TokenKind::Comma) {
                            eprintln!("not comma, set expect_pat to false");
                            expect_pat = false;
                        } else {
                            eprintln!("comma, skipping it");
                            iter.next();
                        }
                    }
                }
            }

            eprintln!("end parsing pat array with pats = {:?}", &pats);

            ASTMatchPatternKind::Array(pats)
        }
        TokenKind::Range => {
            eprintln!("parsing pat rest");
            iter.next();
            ASTMatchPatternKind::Rest
        }
        _ => {
            eprintln!("parsing pat value");
            ASTMatchPatternKind::Value(parse_value(iter, None))
        }
    };

    eprintln!("end parsing pattern");

    temp
}
