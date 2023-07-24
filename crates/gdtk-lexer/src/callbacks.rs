use std::{fmt::Debug, ops::Neg, str::FromStr};

use crate::token::Token;

pub fn trim_comment<'a>(lex: &logos::Lexer<'a, Token<'a>>) -> &'a str {
    let mut slc = &lex.slice()[1..];

    if let Some(s) = slc.strip_suffix('\r') {
        // windoge
        slc = s;
    }

    slc
}

pub fn parse_integer<'a>(lex: &logos::Lexer<'a, Token<'a>>) -> i64 {
    parse_number(lex.slice())
}

pub fn parse_float<'a>(lex: &logos::Lexer<'a, Token<'a>>) -> f64 {
    parse_number(lex.slice())
}

fn parse_number<T>(slc: &str) -> T
where
    T: FromStr + Neg<Output = T>,
    T::Err: Debug,
{
    let minus_count = slc.chars().take_while(|c| *c == '-').count();
    let negative = minus_count % 2 == 1;

    let slc = slc.trim_start_matches('-');

    let mut int: T = slc.parse().unwrap();

    if negative {
        int = -int;
    }

    int
}

pub fn trim_string<'a>(lex: &logos::Lexer<'a, Token<'a>>) -> &'a str {
    lex.slice()
        .strip_prefix('\"')
        .unwrap()
        .strip_suffix('\"')
        .unwrap()
}
