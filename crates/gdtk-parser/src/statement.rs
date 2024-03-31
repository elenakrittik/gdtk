use std::iter::Peekable;

use gdtk_ast::poor::{
    ASTMatchPattern, ASTMatchPatternKind, ASTStatement, ASTValue, ASTVariable,
    ASTVariableKind, CodeBlock,
};
use gdtk_lexer::{Token, TokenKind};

use crate::block::parse_block;
use crate::classes::{parse_class, parse_classname, parse_enum, parse_extends};
use crate::functions::parse_func;
use crate::misc::{parse_annotation, parse_signal};
use crate::utils::{advance_and_parse, expect, expect_blank_prefixed, peek_non_blank};
use crate::expressions::parse_expr;
use crate::variables::parse_variable_body;

pub fn parse_statement<'a>(iter: &mut Peekable<impl Iterator<Item = Token<'a>>>) -> ASTStatement<'a> {
    match peek_non_blank(iter).expect("expected a statement, found EOF").kind {
        TokenKind::Annotation => parse_annotation(iter),
        TokenKind::Assert => ASTStatement::Assert(advance_and_parse(iter, parse_expr)),
        // TODO: figure out how to treat assignments as statements
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
        TokenKind::Var => advance_and_parse(iter, |iter| ASTStatement::Variable(parse_variable_body(iter, ASTVariableKind::Regular))),
        TokenKind::Const => advance_and_parse(iter, |iter| ASTStatement::Variable(parse_variable_body(iter, ASTVariableKind::Constant))),
        TokenKind::Static => {
            iter.next();
            expect_blank_prefixed!(iter, TokenKind::Var, ());
            ASTStatement::Variable(parse_variable_body(iter, ASTVariableKind::Static))
        }
        _ => ASTStatement::Value(parse_expr(iter)),
    }
}

pub fn parse_iflike<'a>(iter: &mut Peekable<impl Iterator<Item = Token<'a>>>) -> (ASTValue<'a>, CodeBlock<'a>) {
    expect_blank_prefixed!(iter, TokenKind::If | TokenKind::Elif | TokenKind::While, ());
    let cond = parse_expr(iter);
    expect_blank_prefixed!(iter, TokenKind::Colon, ());
    let code = parse_block(iter, false);

    (cond, code)
}

pub fn parse_for_loop<'a>(iter: &mut Peekable<impl Iterator<Item = Token<'a>>>) -> ASTStatement<'a> {
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

pub fn parse_match<'a>(iter: &mut Peekable<impl Iterator<Item = Token<'a>>>) -> ASTStatement<'a> {
    expect_blank_prefixed!(iter, TokenKind::Match, ());
    let expr = parse_expr(iter);
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

pub fn parse_pat<'a>(iter: &mut Peekable<impl Iterator<Item = Token<'a>>>) -> ASTMatchPatternKind<'a> {
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
            ASTMatchPatternKind::Value(parse_expr(iter))
        }
    };

    temp
}
