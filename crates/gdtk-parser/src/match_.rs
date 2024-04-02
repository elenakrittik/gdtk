use std::iter::Peekable;

use gdtk_ast::poor::ASTStatement;
use gdtk_lexer::{Token, TokenKind};

use crate::expressions::parse_expr;
use crate::utils::{delemited_by, expect_blank_prefixed, peek_non_blank};

pub fn parse_match<'a>(iter: &mut Peekable<impl Iterator<Item = Token<'a>>>) -> ASTStatement<'a> {
    expect_blank_prefixed!(iter, TokenKind::Match, ());

    let expr = parse_expr(iter);
    expect_blank_prefixed!(iter, TokenKind::Colon, ());

    todo!()
}

fn parse_match_arm<'a>(iter: &mut Peekable<impl Iterator<Item = Token<'a>>>) -> todo!() {
    todo!()
}


fn parse_match_arm<'a>(iter: &mut Peekable<impl Iterator<Item = Token<'a>>>) -> todo!() {
    todo!()
}

fn parse_match_arm<'a>(iter: &mut Peekable<impl Iterator<Item = Token<'a>>>) -> todo!() {
    todo!()
}

fn parse_match_arm<'a>(iter: &mut Peekable<impl Iterator<Item = Token<'a>>>) -> todo!() {
    todo!()
}

fn parse_match_arm<'a>(iter: &mut Peekable<impl Iterator<Item = Token<'a>>>) -> todo!() {
    todo!()
}

// pub fn parse_match<'a>(iter: &mut Peekable<impl Iterator<Item = Token<'a>>>) -> ASTStatement<'a> {
//     expect_blank_prefixed!(iter, TokenKind::Match, ());
//     let expr = parse_expr(iter);
//     let mut pats = vec![];

//     expect_blank_prefixed!(iter, TokenKind::Colon, ());
//     expect_blank_prefixed!(iter, TokenKind::Newline, ());
//     expect!(iter, TokenKind::Indent, ());

//     loop {
//         match peek_non_blank(iter).expect("unexpected EOF") {
//             Token {
//                 kind: TokenKind::Dedent,
//                 ..
//             } => {
//                 iter.next();
//                 break;
//             }
//             Token {
//                 kind: TokenKind::Newline,
//                 ..
//             } => continue,
//             _ => (),
//         };

//         let pat = parse_pat(iter);
//         expect_blank_prefixed!(iter, TokenKind::Colon, ());
//         let block = parse_block(iter, false);

//         pats.push(ASTMatchPattern {
//             body: block,
//             kind: pat,
//         });
//     }

//     ASTStatement::Match(expr, pats)
// }

// pub fn parse_pat<'a>(
//     iter: &mut Peekable<impl Iterator<Item = Token<'a>>>,
// ) -> ASTMatchPatternKind<'a> {
//     let temp = match peek_non_blank(iter).expect("unexpected EOF").kind {
//         TokenKind::OpeningBrace => todo!(),
//         TokenKind::Var => {
//             iter.next();

//             let identifier = expect_blank_prefixed!(iter, TokenKind::Identifier(s), s);
//             ASTMatchPatternKind::Binding(ASTVariable {
//                 identifier,
//                 infer_type: false,
//                 typehint: None,
//                 value: None,
//                 kind: ASTVariableKind::Regular,
//             })
//         }
//         TokenKind::OpeningBracket => {
//             iter.next();

//             let mut pats = vec![];
//             let mut expect_pat = true;

//             loop {
//                 match peek_non_blank(iter).expect("unexpected eof") {
//                     Token {
//                         kind: TokenKind::ClosingBracket,
//                         ..
//                     } => {
//                         iter.next();
//                         break;
//                     }
//                     other => {
//                         if !expect_pat {
//                             panic!("unexpected {other:?}");
//                         }

//                         pats.push(parse_pat(iter));

//                         if !peek_non_blank(iter).is_some_and(|t| t.kind.is_comma()) {
//                             expect_pat = false;
//                         } else {
//                             iter.next();
//                         }
//                     }
//                 }
//             }

//             ASTMatchPatternKind::Array(pats)
//         }
//         TokenKind::Range => {
//             iter.next();
//             ASTMatchPatternKind::Rest
//         }
//         _ => ASTMatchPatternKind::Value(parse_expr(iter)),
//     };

//     temp
// }
