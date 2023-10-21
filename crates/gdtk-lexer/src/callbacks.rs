use std::{fmt::Debug, ops::Neg, str::FromStr};

use gdtk_diag::Diagnostic;

use crate::{
    error::{Error, IntoDiag, WithSpan},
    token::Token,
};

static mut STYLE: IndentStyle = IndentStyle::Unknown;
static mut INDENT_LENGTH: Option<u8> = None;
static mut DIAGS: Vec<Diagnostic> = vec![];

#[derive(Default, PartialEq, Clone)]
enum IndentStyle {
    #[default]
    Unknown,
    Spaces,
    Tabs,
}

fn get_style(blank: &str) -> Result<IndentStyle, Error> {
    if blank.chars().all(|c| c == ' ') {
        Ok(IndentStyle::Spaces)
    } else if blank.chars().all(|c| c == '\t') {
        Ok(IndentStyle::Tabs)
    } else {
        Err(Error::MixedIndent)
    }
}

unsafe fn footgun<'a, T>(reference: &'a T) -> &'a mut T {
    let const_ptr = reference as *const T;
    let mut_ptr = const_ptr as *mut T;
    &mut *mut_ptr
}

// quick & dirty, exactly what i need.
pub unsafe fn check_indent_style<'a>(
    lex: &mut logos::Lexer<'a, Token<'a>>,
) -> Result<&'a str, Error> {
    let slc = lex.slice();
    let style = get_style(slc)?;

    if matches!(INDENT_LENGTH, None) {
        INDENT_LENGTH = Some(slc.len() as u8); // if someone has 64-level indent, then it's their problem
    }

    if matches!(STYLE, IndentStyle::Unknown) {
        STYLE = style;
        return Ok(slc);
    }

    if STYLE != style {
        let err = match style {
            IndentStyle::Spaces => {
                //inplace_replace(slc, ' ', '\t');

                Error::SpaceIndent
            }
            IndentStyle::Tabs => {
                //inplace_replace(slc, '\t', ' ');

                Error::TabIndent
            }
            _ => unreachable!(),
        };

        DIAGS.push(err.with_span(lex.span()).into_diag());

        Ok(slc)
    } else {
        Ok(slc)
    }
}

/*
fn inplace_replace(s: &mut str, from: char, to: char) {
    let bytes = unsafe { s.as_bytes_mut() };
    let mut to_replace = vec![];

    for (idx, byte) in bytes.iter().enumerate() {
        if byte == &(from as u8) {
            to_replace.push(idx);
        }
    }

    for idx in bytes
        .iter()
        .enumerate()
        .filter(|(_, b)| *b == &(from as u8))
        .map(|(i, _)| i)
    {
        idx;
    }
}
*/

pub fn trim_comment<'a>(lex: &logos::Lexer<'a, Token<'a>>) -> &'a str {
    let mut slc = &lex.slice()[1..];

    // windoge
    if let Some(s) = slc.strip_suffix('\r') {
        slc = s;
    }

    slc
}

pub fn parse_integer<'a>(lex: &logos::Lexer<'a, Token<'a>>) -> i64 {
    // actually always u64
    parse_number(lex.slice())
}

pub fn parse_float<'a>(lex: &logos::Lexer<'a, Token<'a>>) -> f64 {
    parse_number(lex.slice())
}

pub fn parse_hex<'a>(lex: &logos::Lexer<'a, Token<'a>>) -> u64 {
    let slc = lex.slice()[2..].replace('_', "").to_lowercase();
    let mut mantissa = 0;
    let mut result: u64 = 0;

    for c in slc.chars().rev() {
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

        result += val * (16u64.pow(mantissa));
        mantissa += 1;
    }

    result
}

pub fn parse_binary<'a>(lex: &logos::Lexer<'a, Token<'a>>) -> u64 {
    let slc = lex.slice()[2..].replace('_', "");
    u64::from_str_radix(&slc, 2).unwrap()
}

pub fn parse_e_notation<'a>(lex: &logos::Lexer<'a, Token<'a>>) -> f64 {
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

pub fn parse_bool<'a>(lex: &logos::Lexer<'a, Token<'a>>) -> bool {
    lex.slice().parse().unwrap()
}

// TODO: come up with better names for these

pub fn strip_quotes<'a>(lex: &logos::Lexer<'a, Token<'a>>) -> Result<&'a str, Error> {
    let slice = lex.slice();

    strip_quotes_impl(slice)
}

pub fn strip_prefix_and_quotes<'a>(
    lex: &logos::Lexer<'a, Token<'a>>,
    prefix: char,
) -> Result<&'a str, Error> {
    strip_quotes_impl(lex.slice().strip_prefix(prefix).unwrap())
}

fn strip_quotes_impl(slice: &str) -> Result<&str, Error> {
    let double = slice.contains('"');
    let single = slice.contains('\'');

    dbg!("parsing str!", &slice);

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

    let c = slice.chars().nth(0).unwrap();

    if c == '"' {
        return Err(Error::UnclosedDoubleStringLiteral);
    }

    if c == '\'' {
        return Err(Error::UnclosedSingleStringLiteral);
    }

    unreachable!()
}
