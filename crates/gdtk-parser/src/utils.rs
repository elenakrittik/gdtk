// TODO: refactor some stuff to utilize new option to .peek()

use std::iter::Peekable;

use gdtk_lexer::{Token, TokenKind};

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

pub fn next_non_blank<'a>(iter: &mut Peekable<impl Iterator<Item = Token<'a>>>) -> Token<'a> {
    loop {
        if let Some(token) = iter.next() {
            match token.kind {
                TokenKind::Blank(_) => (),
                _ => break token,
            }
        } else {
            panic!("unexpected EOF");
        }
    }
}

/// Parses a list of values (as defined by the passed callback) separated by the specified delimiter.
/// ``stop_at`` is used to know when to stop looking for new values.
pub fn delemited_by<'a, I, V>(
    iter: &mut Peekable<I>,
    delimiter: TokenKind<'a>,
    stop_at: &[TokenKind<'a>],
    mut callback: impl FnMut(&mut Peekable<I>) -> V,
) -> Vec<V>
where
    I: Iterator<Item = Token<'a>>,
{
    let mut values = vec![];

    while iter.peek().is_some_and(|t| !(stop_at.iter().any(|k| k.same_as(&t.kind)))) {
        values.push(callback(iter));

        if iter.peek().is_some_and(|t| t.kind.same_as(&delimiter)) {
            iter.next();
        }
    }

    values
}

/// Calls ``iter.next()``, then ``callback(iter)``.
pub fn advance_and_parse<'a, I, V>(
    iter: &mut Peekable<I>,
    mut callback: impl FnMut(&mut Peekable<I>) -> V
) -> V
where
    I: Iterator<Item = Token<'a>>,
{
    iter.next();
    callback(iter)
}
