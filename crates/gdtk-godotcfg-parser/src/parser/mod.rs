use logos::Logos;

use crate::{
    ast::{Line, Value},
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
            TokenKind::Identifier(_) | TokenKind::Path(_) => self.parse_parameter(),
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
        self.tokens.next_ok()?.expect_opening_bracket()?;

        let identifier = self.tokens.next_ok()?.expect_identifier()?;

        self.tokens.next_ok()?.expect_closing_bracket()?;

        Ok(Line::Section(identifier))
    }

    fn parse_parameter(&mut self) -> Result<Line<'a>, Error<'a>> {
        let identifier = self.tokens.next_ok()?.expect_identifier_like()?;

        self.tokens.next_ok()?.expect_assignment()?;

        let value = self.parse_value()?;

        Ok(Line::Parameter(identifier, value))
    }

    fn parse_value(&mut self) -> Result<Value<'a>, Error<'a>> {
        match self.tokens.peek_ok()?.kind {
            TokenKind::String(_) => Ok(Value::String(self.tokens.next_ok()?.expect_string()?)),
            TokenKind::Integer(_) => Ok(Value::Integer(self.tokens.next_ok()?.expect_integer()?)),
            TokenKind::Float(_) => Ok(Value::Float(self.tokens.next_ok()?.expect_float()?)),
            TokenKind::Boolean(_) => Ok(Value::Boolean(self.tokens.next_ok()?.expect_boolean()?)),
            TokenKind::Null => Ok(Value::Null),
            TokenKind::Identifier(_) => self.parse_any_object(),
            TokenKind::OpeningBracket => self.parse_array(),
            TokenKind::OpeningBrace => self.parse_map(),
            _ => Err(Error::Unexpected(self.tokens.next_ok()?, "a value")),
        }
    }
}
