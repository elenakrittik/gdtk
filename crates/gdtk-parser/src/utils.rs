// TODO: refactor some stuff to utilize new option to .peek()

use std::iter::Peekable;

use gdtk_ast::poor::{ASTValue, ASTVariable};
use gdtk_lexer::{Token, TokenKind};

use crate::{expressions::parse_expr, variables::parse_variable};

pub fn collect_params<'a, T>(iter: &mut Peekable<T>) -> Vec<ASTVariable<'a>>
where
    T: Iterator<Item = Token<'a>>,
{
    let mut parameters = vec![];

    expect_blank_prefixed!(iter, TokenKind::OpeningParenthesis, ());

    if !peek_non_blank(iter).is_some_and(|t| t.kind.is_closing_parenthesis()) {
        loop {
            if !peek_non_blank(iter).is_some_and(|t| matches!(t.kind, TokenKind::Identifier(_))) {
                panic!("unexpected {:?}, expected function parameter", iter.next());
            }

            let param = parse_variable(iter, gdtk_ast::poor::ASTVariableKind::FunctionParameter);
            parameters.push(param);

            match peek_non_blank(iter).expect("unexpected EOF").kind {
                TokenKind::Comma => {
                    iter.next();
                    continue;
                }
                TokenKind::ClosingParenthesis => {
                    iter.next();
                    break;
                }
                ref other => panic!("unexpected {other:?}, expected a comma or a closing parenthesis"),
            }
        }
    } else {
        iter.next();
    }

    parameters
}

pub macro expect($iter:expr, $variant:pat, $ret:expr) {{
    type Token<'a> = ::gdtk_lexer::Token<'a>;

    match $iter.next() {
        Some(Token { kind: $variant, .. }) => $ret,
        other => panic!("expected {}, found {other:?}", stringify!($variant)),
    }
}}

pub macro expect_blank_prefixed($iter:expr, $variant:pat, $ret:expr) {{
    type TokenKind<'a> = ::gdtk_lexer::TokenKind<'a>;

    loop {
        if let Some(token) = $iter.next() {
            match token.kind {
                TokenKind::Blank(_) => (),
                $variant => break $ret,
                _ => panic!("expected {}, found {token:?}", stringify!($variant)),
            }
        } else {
            panic!("unexpected EOF");
        }
    }
}}

// pub macro peek_non_blank($iter:expr) {{
//     type TokenKind<'a> = ::gdtk_lexer::TokenKind<'a>;

//     loop {
//         if let Some(token) = $iter.peek() {
//             match token.kind {
//                 TokenKind::Blank(_) => {
//                     $iter.next();
//                 }
//                 _ => break token,
//             }
//         } else {
//             panic!("unexpected EOF");
//         }
//     }
// }}

pub fn peek_non_blank<'a>(iter: &mut Peekable<impl Iterator<Item = Token<'a>>>) -> Option<&Token<'a>> {
    loop {
        if iter.peek().is_some_and(|t| matches!(t.kind, TokenKind::Blank(_))) {
            iter.next();
        } else if iter.peek().is_some() {
            break Some(iter.peek().unwrap());
        } else {
            break None;
        }
    }
}

pub macro next_non_blank($iter:expr) {{
    type TokenKind<'a> = ::gdtk_lexer::TokenKind<'a>;

    loop {
        if let Some(token) = $iter.next() {
            match token.kind {
                TokenKind::Blank(_) => (),
                _ => break token,
            }
        } else {
            panic!("unexpected EOF");
        }
    }
}}

/// Collects values surrounded by parentheses or brackets
pub fn collect_values<'a, T>(iter: &mut Peekable<T>, skip_first: bool) -> Vec<ASTValue<'a>>
where
    T: Iterator<Item = Token<'a>>,
{
    let mut values = vec![];

    if !skip_first {
        expect_blank_prefixed!(iter, TokenKind::OpeningParenthesis | TokenKind::OpeningBracket, ());
    }

    if !peek_non_blank(iter).is_some_and(|t| t.kind.is_closing_parenthesis() || t.kind.is_opening_bracket()) {
        loop {
            let value = parse_expr(iter);
            values.push(value);

            match peek_non_blank(iter).expect("unexpected EOF").kind {
                TokenKind::Comma => {
                    iter.next();
                    continue;
                }
                TokenKind::ClosingParenthesis | TokenKind::ClosingBracket => {
                    iter.next();
                    break;
                }
                ref other => panic!("unexpected {other:?}, expected a comma or a closing parenthesis"),
            }
        }
    } else {
        iter.next();
    }

    values
}
