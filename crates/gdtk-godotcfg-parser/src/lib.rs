use std::fmt::Debug;

use logos::Logos;

use crate::{
    parser::Parser,
    token::{Token, TokenKind},
    utils::PeekableResultIterator,
};

pub mod ast;
pub mod error;
pub mod parser;
pub mod token;
pub mod utils;

pub fn lexer(source: &str) -> impl PeekableResultIterator<Item = Token<'_>> + Debug {
    TokenKind::lexer(source)
        .spanned()
        .filter_map(|(result, span)| result.ok().zip(Some(span)))
        .map(|(token, span)| Token::new(token, span))
        .peekable()
}

pub fn parser(source: &str) -> Parser<impl PeekableResultIterator<Item = Token<'_>> + Debug> {
    Parser {
        tokens: lexer(source),
        had_error: false,
    }
}
