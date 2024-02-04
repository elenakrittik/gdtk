use std::{fmt::Debug, ops::Neg, str::FromStr};

use crate::{error::Error, token::TokenKind};

pub fn trim_comment<'a>(lex: &logos::Lexer<'a, TokenKind<'a>>) -> &'a str {
    let mut slc = &lex.slice()[1..];

    // windoge
    if let Some(s) = slc.strip_suffix('\r') {
        slc = s;
    }

    slc
}

pub fn parse_integer<'a>(lex: &logos::Lexer<'a, TokenKind<'a>>) -> i64 {
    // actually always u64
    parse_number(lex.slice())
}

pub fn parse_float<'a>(lex: &logos::Lexer<'a, TokenKind<'a>>) -> f64 {
    parse_number(lex.slice())
}

pub fn parse_hex<'a>(lex: &logos::Lexer<'a, TokenKind<'a>>) -> u64 {
    let slc = lex.slice()[2..].replace('_', "").to_lowercase();
    let mut result: u64 = 0;

    for (mantissa, c) in slc.chars().rev().enumerate() {
        let val = match c {
            '0' => 0,
            '1' => 1,
            '2' => 2,
            '3' => 3,
            '4' => 4,
            '5' => 5,
            '6' => 6,
            '7' => 7,
            '8' => 8,
            '9' => 9,
            'a' => 10,
            'b' => 11,
            'c' => 12,
            'd' => 13,
            'e' => 14,
            'f' => 15,
            _ => panic!(
                "Somehow, a hex literal had a character other than [0-9abcdef]: {}. Fix it.",
                c
            ),
        };

        result += val * (16u64.pow(mantissa as u32));
    }

    result
}

pub fn parse_binary<'a>(lex: &logos::Lexer<'a, TokenKind<'a>>) -> u64 {
    let slc = lex.slice()[2..].replace('_', "");
    u64::from_str_radix(&slc, 2).unwrap()
}

pub fn parse_e_notation<'a>(lex: &logos::Lexer<'a, TokenKind<'a>>) -> f64 {
    let slc = lex.slice().replace('_', "");

    parse_number(&slc)
}

fn parse_number<T>(slc: &str) -> T
where
    T: FromStr + Neg<Output = T>,
    T::Err: Debug,
{
    let slc = slc.replace('_', "");
    let minus_count = slc.chars().take_while(|c| *c == '-').count();
    let negative = minus_count % 2 == 1;

    let slc = slc.trim_start_matches('-');

    let mut int: T = slc.parse().unwrap();

    if negative {
        int = -int;
    }

    int
}

pub fn parse_bool<'a>(lex: &logos::Lexer<'a, TokenKind<'a>>) -> bool {
    lex.slice().parse().unwrap()
}

// TODO: come up with better names for these

pub fn strip_quotes<'a>(lex: &logos::Lexer<'a, TokenKind<'a>>) -> Result<&'a str, Error> {
    let slice = lex.slice();

    strip_quotes_impl(slice)
}

pub fn strip_prefix_and_quotes<'a>(
    lex: &logos::Lexer<'a, TokenKind<'a>>,
    prefix: char,
) -> Result<&'a str, Error> {
    strip_quotes_impl(lex.slice().strip_prefix(prefix).unwrap())
}

fn strip_quotes_impl(slice: &str) -> Result<&str, Error> {
    let double = slice.contains('"');
    let single = slice.contains('\'');

    if double && !single {
        let slc = slice.strip_prefix('"').unwrap();

        if let Some(s) = slc.strip_suffix('"') {
            return Ok(s);
        }
    }

    if single && !double {
        let slc = slice.strip_prefix('\'').unwrap();

        if let Some(s) = slc.strip_suffix('\'') {
            return Ok(s);
        }
    }

    let c = slice.chars().next().unwrap();

    if c == '"' {
        return Err(Error::UnclosedDoubleStringLiteral);
    }

    if c == '\'' {
        return Err(Error::UnclosedSingleStringLiteral);
    }

    unreachable!()
}
