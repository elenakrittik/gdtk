use logos::Logos;

use crate::{
    ast::Line,
    error::Error,
    token::{Token, TokenKind},
    utils::ResultIterator,
};

#[derive(Debug)]
pub struct Parser<I> {
    tokens: I,
}

impl<I> Parser<I> {
    pub fn new<'a>(source: &'a str) -> Parser<impl ResultIterator<Item = Token<'a>>> {
        Parser {
            tokens: TokenKind::lexer(source)
                .spanned()
                .filter_map(|(result, span)| result.ok().zip(Some(span)))
                .map(|(token, span)| Token::new(token, span))
                .peekable(),
        }
    }
}

impl<'a, I> Iterator for Parser<I>
where
    I: ResultIterator<Item = Token<'a>>,
{
    type Item = Result<Line<'a>, Error<'a>>;

    fn next(&mut self) -> Option<Self::Item> {
        Some(match self.tokens.peek()?.kind {
            TokenKind::Comment(comment) => Ok(Line::Comment(comment)),
            TokenKind::OpeningBracket => self.parse_section(),
            _ => Err(Error::Unexpected(
                self.tokens.next()?,
                "a comment, a section, or a parameter",
            )),
        })
    }
}

impl<'a, I> Parser<I>
where
    I: ResultIterator<Item = Token<'a>>,
{
    fn parse_section(&mut self) -> Result<Line<'a>, Error<'a>> {
        debug_assert!(self.tokens.next_ok()?.kind.is_opening_bracket());

        let identifier = self.tokens.next_ok()?.into_identifier()?;

        todo!()
    }
}
