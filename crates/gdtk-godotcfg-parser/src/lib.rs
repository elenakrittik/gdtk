use logos::Logos;

use crate::{
    parser::Parser,
    token::{Token, TokenKind},
    utils::ResultIterator,
};

pub mod ast;
pub mod error;
pub mod parser;
pub mod token;
pub mod utils;

pub fn parser(source: &str) -> Parser<impl ResultIterator<Item = Token<'_>>> {
    Parser {
        tokens: TokenKind::lexer(source)
            .spanned()
            .filter_map(|(result, span)| result.ok().zip(Some(span)))
            .map(|(token, span)| Token::new(token, span))
            .peekable(),
    }
}
