use std::str::FromStr;

use crate::token::TokenKind;

#[derive(Default)]
pub struct State {
    pub(crate) paren_open: isize,
}

pub(crate) fn mut_open_paren<'a, const N: isize>(
    lexer: &mut logos::Lexer<'a, TokenKind<'a>>,
) {
    lexer.extras.paren_open += N;
}

pub(crate) fn convert<'a, T: std::str::FromStr>(
    lexer: &logos::Lexer<'a, TokenKind<'a>>,
) -> logos::Filter<T> {
    match FromStr::from_str(&lexer.slice().replace('_', "")) {
        Ok(val) => logos::Filter::Emit(val),
        Err(_) => logos::Filter::Skip,
    }
}

pub(crate) fn convert_radix<'a, const R: u32>(
    lexer: &logos::Lexer<'a, TokenKind<'a>>,
) -> logos::Filter<u64> {
    match u64::from_str_radix(&lexer.slice()[2..].replace('_', ""), R) {
        Ok(val) => logos::Filter::Emit(val),
        Err(_) => logos::Filter::Skip,
    }
}

pub(crate) fn trim_quotes<'a, const SKIP_PREFIX: bool>(
    lexer: &logos::Lexer<'a, TokenKind<'a>>,
) -> &'a str {
    if SKIP_PREFIX {
        &lexer.slice()[1..]
    } else {
        lexer.slice()
    }
    .trim_matches(['\'', '"'])
}
