use std::fmt::Debug;

use tracing::trace;

use crate::{
    ast::{Line, Value},
    error::Error,
    token::{Token, TokenKind},
    utils::ResultIterator,
};

/// A GodotCfg parser.
///
/// GodofCfg is an unofficial name for textual configuration
/// format Godot uses for it's `.godot`, `.tscn` and `.tres`
/// files.
///
/// The parser is a recursive-descent parser modelled as an
/// iterator. This is possible (and makes sense) because
/// GodotCfg is a "flat" format (i.e., statements cannot
/// appear inside other statements), so each iteration
/// intuitively maps to an "entry" (a "line", a statement).
///
/// The primary (and only) way to construct a [Parser] is
/// through [crate::parser].
#[derive(Debug)]
pub struct Parser<I> {
    pub(crate) tokens: I,
    pub(crate) had_error: bool,
}

impl<'a, I> Iterator for Parser<I>
where
    I: ResultIterator<Item = Token<'a>> + Debug,
{
    type Item = Result<Line<'a>, Error<'a>>;

    #[tracing::instrument(skip(self))]
    fn next(&mut self) -> Option<Self::Item> {
        trace!("Parser::next");

        if self.had_error {
            return None;
        }

        let result = match self.tokens.peek()?.kind {
            TokenKind::Comment(_) => self.parse_comment(),
            TokenKind::OpeningBracket => self.parse_section(),
            TokenKind::Identifier(_) | TokenKind::Path(_) => self.parse_parameter(),
            _ => Err(Error::Unexpected(
                self.tokens.next()?,
                "a comment, a section, or a parameter",
            )),
        };

        if result.is_err() {
            self.had_error = true;
        }

        Some(result)
    }
}

impl<'a, I> Parser<I>
where
    I: ResultIterator<Item = Token<'a>> + Debug,
{
    #[tracing::instrument(skip(self))]
    fn parse_comment(&mut self) -> Result<Line<'a>, Error<'a>> {
        trace!("Parser::parse_comment");

        Ok(Line::Comment(self.tokens.next_ok()?.expect_comment()?))
    }

    #[tracing::instrument(skip(self))]
    fn parse_section(&mut self) -> Result<Line<'a>, Error<'a>> {
        trace!("Parser::parse_section");

        trace!("Parser::parse_section - expect_opening_bracket");

        self.tokens.next_ok()?.expect_opening_bracket()?;

        trace!("Parser::parse_section - expect_identifier");

        let identifier = self.tokens.next_ok()?.expect_identifier()?;

        let mut parameters = Vec::new();

        while !self.tokens.peek_ok()?.is_closing_bracket() {
            let parameter = self.tokens.next_ok()?.expect_identifier()?;

            self.tokens.next_ok()?.expect_assignment()?;

            let value = self.parse_value()?;

            parameters.push((parameter, value));
        }

        trace!("Parser::parse_section - expect_closing_bracket");

        self.tokens.next_ok()?.expect_closing_bracket()?;

        Ok(Line::Section(identifier, parameters))
    }

    #[tracing::instrument(skip(self))]
    fn parse_parameter(&mut self) -> Result<Line<'a>, Error<'a>> {
        trace!("Parser::parse_parameter");

        trace!("Parser::parse_parameter - expect_path_like");

        let path = self.tokens.next_ok()?.expect_path_like()?;

        trace!("Parser::parse_parameter - expect_assignment");

        self.tokens.next_ok()?.expect_assignment()?;

        trace!("Parser::parse_parameter - expect_value");

        let value = self.parse_value()?;

        Ok(Line::Parameter(path, value))
    }

    #[tracing::instrument(skip(self))]
    fn parse_value(&mut self) -> Result<Value<'a>, Error<'a>> {
        trace!("Parser::parse_value");

        match self.tokens.peek_ok()?.kind {
            TokenKind::String(_) => self.parse_string(),
            TokenKind::Integer(_) => self.parse_integer(),
            TokenKind::Float(_) => self.parse_float(),
            TokenKind::Boolean(_) => self.parse_boolean(),
            TokenKind::Null => self.parse_null(),
            TokenKind::Identifier(_) => self.parse_object_or_instance(),
            TokenKind::OpeningBracket => self.parse_array(),
            TokenKind::OpeningBrace => self.parse_map(),
            _ => Err(Error::Unexpected(self.tokens.next_ok()?, "a value")),
        }
    }

    #[tracing::instrument(skip(self))]
    fn parse_string(&mut self) -> Result<Value<'a>, Error<'a>> {
        trace!("Parser::parse_string");

        Ok(Value::String(self.tokens.next_ok()?.expect_string()?))
    }

    #[tracing::instrument(skip(self))]
    fn parse_integer(&mut self) -> Result<Value<'a>, Error<'a>> {
        trace!("Parser::parse_integer");

        Ok(Value::Integer(self.tokens.next_ok()?.expect_integer()?))
    }

    #[tracing::instrument(skip(self))]
    fn parse_float(&mut self) -> Result<Value<'a>, Error<'a>> {
        trace!("Parser::parse_float");

        Ok(Value::Float(self.tokens.next_ok()?.expect_float()?))
    }

    #[tracing::instrument(skip(self))]
    fn parse_boolean(&mut self) -> Result<Value<'a>, Error<'a>> {
        trace!("Parser::parse_boolean");

        Ok(Value::Boolean(self.tokens.next_ok()?.expect_boolean()?))
    }

    #[tracing::instrument(skip(self))]
    fn parse_null(&mut self) -> Result<Value<'a>, Error<'a>> {
        trace!("Parser::parse_null");

        self.tokens.next_ok()?.expect_null()?;

        Ok(Value::Null)
    }

    #[tracing::instrument(skip(self))]
    fn parse_array(&mut self) -> Result<Value<'a>, Error<'a>> {
        trace!("Parser::parse_array");

        self.tokens.next_ok()?.expect_opening_bracket()?;

        let mut values = Vec::new();

        while !self.tokens.peek_ok()?.is_closing_bracket() {
            values.push(self.parse_value()?);

            if self.tokens.peek_ok()?.is_comma() {
                self.tokens.next_ok()?;
            }
        }

        self.tokens.next_ok()?.expect_closing_bracket()?;

        Ok(Value::Array(values))
    }

    #[tracing::instrument(skip(self))]
    fn parse_map(&mut self) -> Result<Value<'a>, Error<'a>> {
        trace!("Parser::parse_map");

        self.tokens.next_ok()?.expect_opening_brace()?;

        let mut pairs = Vec::new();

        while !self.tokens.peek_ok()?.is_closing_brace() {
            let key = self.parse_value()?;

            self.tokens.next_ok()?.expect_colon()?;

            let value = self.parse_value()?;

            pairs.push((key, value));

            if self.tokens.peek_ok()?.is_comma() {
                self.tokens.next_ok()?;
            }
        }

        self.tokens.next_ok()?.expect_closing_brace()?;

        Ok(Value::Map(pairs))
    }

    #[tracing::instrument(skip(self))]
    fn parse_object_or_instance(&mut self) -> Result<Value<'a>, Error<'a>> {
        trace!("Parser::parse_object_or_instance");

        let identifier = self.tokens.next_ok()?.expect_identifier()?;

        match identifier {
            "Object" => self.parse_object(),
            other => self.parse_object_instance(other),
        }
    }

    #[tracing::instrument(skip(self))]
    fn parse_object_instance(&mut self, identifier: &'a str) -> Result<Value<'a>, Error<'a>> {
        trace!("Parser::parse_object_instance");

        self.tokens.next_ok()?.expect_opening_parenthesis()?;

        let mut values = Vec::new();

        while !self.tokens.peek_ok()?.is_closing_parenthesis() {
            values.push(self.parse_value()?);

            if self.tokens.peek_ok()?.is_comma() {
                self.tokens.next_ok()?;
            }
        }

        self.tokens.next_ok()?.expect_closing_parenthesis()?;

        Ok(Value::ObjectInstance(identifier, values))
    }

    #[tracing::instrument(skip(self))]
    fn parse_object(&mut self) -> Result<Value<'a>, Error<'a>> {
        trace!("Parser::parse_object");

        self.tokens.next_ok()?.expect_opening_parenthesis()?;

        let identifier = self.tokens.next_ok()?.expect_identifier()?;

        if self.tokens.peek_ok()?.is_comma() {
            self.tokens.next_ok()?;
        }

        let mut properties = Vec::new();

        while !self.tokens.peek_ok()?.is_closing_parenthesis() {
            let property = self.tokens.next_ok()?.expect_string()?;

            self.tokens.next_ok()?.expect_colon()?;

            let value = self.parse_value()?;

            properties.push((property, value));

            if self.tokens.peek_ok()?.is_comma() {
                self.tokens.next_ok()?;
            }
        }

        self.tokens.next_ok()?.expect_closing_parenthesis()?;

        Ok(Value::Object(identifier, properties))
    }
}
