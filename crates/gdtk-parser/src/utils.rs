// TODO: refactor some stuff to utilize new option to .peek()

use std::iter::Peekable;

use gdtk_ast::poor::ASTVariable;
use gdtk_lexer::{Token, TokenKind};

use crate::variables::parse_variable;

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

            match peek_non_blank(iter).expect("unexpected EOF") {
                Token {
                    kind: TokenKind::Comma,
                    ..
                } => {
                    iter.next();
                    continue;
                }
                Token {
                    kind: TokenKind::ClosingParenthesis,
                    ..
                } => {
                    iter.next();
                    break;
                }
                other => panic!("unexpected {other:?}, expected a comma or a closing parenthesis"),
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

pub macro collect_args($iter:expr, $opening:pat, $closing:pat) {{
    $crate::utils::expect!($iter, $opening, ());
    $crate::utils::collect_args_raw!($iter, $closing)
}}

pub macro collect_args_raw($iter:expr, $closing:pat) {{
    type TokenKind<'a> = ::gdtk_lexer::TokenKind<'a>;

    let mut args = vec![];
    let mut expect_comma = false;

    while let Some(token) = $iter.next() {
        match &token.kind {
            &TokenKind::Comma => {
                if !expect_comma {
                    panic!("unexpected comma, expected a value");
                }
                expect_comma = false;
            }
            &TokenKind::Blank(_) => (),
            &$closing => break,
            other => {
                if expect_comma {
                    panic!("expected comma, got {other:?}");
                }
                args.push($crate::values::parse_value($iter, Some(token)));
                expect_comma = true;
            }
        }
    }

    args
}}
