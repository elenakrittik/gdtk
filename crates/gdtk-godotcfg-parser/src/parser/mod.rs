use logos::Logos;

use crate::{
    ast::Line,
    token::{Token, TokenKind},
    utils::PeekableIterator,
};

#[derive(Debug)]
pub struct Parser<'a, I>
where
    I: PeekableIterator<Item = Token<'a>>,
{
    tokens: I,
}

impl<'a, I> Parser<'a, I>
where
    I: PeekableIterator<Item = Token<'a>>,
{
    pub fn new(source: &'a str) -> Parser<'a, impl PeekableIterator<Item = Token<'a>>> {
        Self {
            tokens: TokenKind::lexer(source)
                .spanned()
                .filter_map(|(result, span)| result.ok().zip(Some(span)))
                .map(|(token, span)| Token::new(token, span)),
        }
    }
}

impl<'a, I> Iterator for Parser<'a, I>
where
    I: PeekableIterator<Item = Token<'a>>,
{
    type Item = Line<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.tokens.peek()?.kind {
            _ => todo!(),
        };
    }
}
