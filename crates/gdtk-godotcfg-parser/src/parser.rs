use std::fmt::Debug;

use tracing::trace;

use crate::{
    ast::{Line, Value},
    error::Error,
    token::{Token, TokenKind},
    utils::PeekableResultIterator,
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
    I: PeekableResultIterator<Item = Token<'a>> + Debug,
{
    type Item = Result<Line<'a>, Error<'a>>;

    #[tracing::instrument(skip(self), level = tracing::Level::TRACE)]
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
    I: PeekableResultIterator<Item = Token<'a>> + Debug,
{
    #[tracing::instrument(skip(self), level = tracing::Level::TRACE)]
    fn parse_comment(&mut self) -> Result<Line<'a>, Error<'a>> {
        trace!("Parser::parse_comment");

        Ok(Line::Comment(self.tokens.next_ok()?.expect_comment()?))
    }

    #[tracing::instrument(skip(self), level = tracing::Level::TRACE)]
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

    #[tracing::instrument(skip(self), level = tracing::Level::TRACE)]
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

    #[tracing::instrument(skip(self), level = tracing::Level::TRACE)]
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

    #[tracing::instrument(skip(self), level = tracing::Level::TRACE)]
    fn parse_string(&mut self) -> Result<Value<'a>, Error<'a>> {
        trace!("Parser::parse_string");

        Ok(Value::String(self.tokens.next_ok()?.expect_string()?))
    }

    #[tracing::instrument(skip(self), level = tracing::Level::TRACE)]
    fn parse_integer(&mut self) -> Result<Value<'a>, Error<'a>> {
        trace!("Parser::parse_integer");

        Ok(Value::Integer(self.tokens.next_ok()?.expect_integer()?))
    }

    #[tracing::instrument(skip(self), level = tracing::Level::TRACE)]
    fn parse_float(&mut self) -> Result<Value<'a>, Error<'a>> {
        trace!("Parser::parse_float");

        Ok(Value::Float(self.tokens.next_ok()?.expect_float()?))
    }

    #[tracing::instrument(skip(self), level = tracing::Level::TRACE)]
    fn parse_boolean(&mut self) -> Result<Value<'a>, Error<'a>> {
        trace!("Parser::parse_boolean");

        Ok(Value::Boolean(self.tokens.next_ok()?.expect_boolean()?))
    }

    #[tracing::instrument(skip(self), level = tracing::Level::TRACE)]
    fn parse_null(&mut self) -> Result<Value<'a>, Error<'a>> {
        trace!("Parser::parse_null");

        self.tokens.next_ok()?.expect_null()?;

        Ok(Value::Null)
    }

    #[tracing::instrument(skip(self), level = tracing::Level::TRACE)]
    fn parse_array(&mut self) -> Result<Value<'a>, Error<'a>> {
        trace!("Parser::parse_array");

        trace!("Parser::parse_array - expect_opening_bracket");

        self.tokens.next_ok()?.expect_opening_bracket()?;

        let mut values = Vec::new();

        while !self.tokens.peek_ok()?.is_closing_bracket() {
            values.push(self.parse_value()?);

            if self.tokens.peek_ok()?.is_comma() {
                self.tokens.next_ok()?;
            }
        }

        trace!("Parser::parse_array - expect_closing_bracket");

        self.tokens.next_ok()?.expect_closing_bracket()?;

        Ok(Value::Array(values))
    }

    #[tracing::instrument(skip(self), level = tracing::Level::TRACE)]
    fn parse_map(&mut self) -> Result<Value<'a>, Error<'a>> {
        trace!("Parser::parse_map");

        trace!("Parser::parse_map - expect_opening_brace");

        self.tokens.next_ok()?.expect_opening_brace()?;

        let mut pairs = Vec::new();

        while !self.tokens.peek_ok()?.is_closing_brace() {
            let key = self.parse_value()?;

            trace!("Parser::parse_map - expect_colon");

            self.tokens.next_ok()?.expect_colon()?;

            let value = self.parse_value()?;

            pairs.push((key, value));

            if self.tokens.peek_ok()?.is_comma() {
                self.tokens.next_ok()?;
            }
        }

        trace!("Parser::parse_map - expect_closing_brace");

        self.tokens.next_ok()?.expect_closing_brace()?;

        Ok(Value::Map(pairs))
    }

    #[tracing::instrument(skip(self), level = tracing::Level::TRACE)]
    fn parse_object_or_instance(&mut self) -> Result<Value<'a>, Error<'a>> {
        trace!("Parser::parse_object_or_instance");

        trace!("Parser::parse_object_or_instance - expect_identifier");

        let identifier = self.tokens.next_ok()?.expect_identifier()?;

        match identifier {
            "Object" => self.parse_object(),
            other => self.parse_object_instance(other),
        }
    }

    #[tracing::instrument(skip(self), level = tracing::Level::TRACE)]
    fn parse_object_instance(&mut self, identifier: &'a str) -> Result<Value<'a>, Error<'a>> {
        trace!("Parser::parse_object_instance");

        trace!("Parser::parse_object_instance - expect_opening_parenthesis");

        self.tokens.next_ok()?.expect_opening_parenthesis()?;

        let mut values = Vec::new();

        while !self.tokens.peek_ok()?.is_closing_parenthesis() {
            values.push(self.parse_value()?);

            if self.tokens.peek_ok()?.is_comma() {
                self.tokens.next_ok()?;
            }
        }

        trace!("Parser::parse_object_instance - expect_closing_parenthesis");

        self.tokens.next_ok()?.expect_closing_parenthesis()?;

        Ok(Value::ObjectInstance(identifier, values))
    }

    #[tracing::instrument(skip(self), level = tracing::Level::TRACE)]
    fn parse_object(&mut self) -> Result<Value<'a>, Error<'a>> {
        trace!("Parser::parse_object");

        trace!("Parser::parse_object - expect_opening_parenthesis");

        self.tokens.next_ok()?.expect_opening_parenthesis()?;

        trace!("Parser::parse_object - expect_identifier");

        let identifier = self.tokens.next_ok()?.expect_identifier()?;

        if self.tokens.peek_ok()?.is_comma() {
            self.tokens.next_ok()?;
        }

        let mut properties = Vec::new();

        while !self.tokens.peek_ok()?.is_closing_parenthesis() {
            trace!("Parser::parse_object - expect_string");

            let property = self.tokens.next_ok()?.expect_string()?;

            trace!("Parser::parse_object - expect_colon");

            self.tokens.next_ok()?.expect_colon()?;

            let value = self.parse_value()?;

            properties.push((property, value));

            if self.tokens.peek_ok()?.is_comma() {
                self.tokens.next_ok()?;
            }
        }

        trace!("Parser::parse_object - expect_closing_parenthesis");

        self.tokens.next_ok()?.expect_closing_parenthesis()?;

        Ok(Value::Object(identifier, properties))
    }
}

#[cfg(test)]
mod tests {
    use crate::ast::*;
    use crate::utils::ResultIterator;

    type Test = Result<(), crate::error::Error<'static>>;

    macro_rules! parse {
        ($input:expr) => {{
            let mut parser = $crate::parser($input);

            parser.next_ok()??
        }};
    }

    macro_rules! parse_val {
        ($input:expr) => {{
            let mut parser = $crate::parser($input);

            parser.parse_value()?
        }};
    }

    #[test]
    fn test_line_comment() -> Test {
        assert_eq!(parse!("; ok"), Line::Comment(" ok"));

        Ok(())
    }

    #[test]
    fn test_line_section() -> Test {
        assert_eq!(parse!("[section]"), Line::Section("section", vec![]));
        assert_eq!(
            parse!("[section param=0]"),
            Line::Section("section", vec![("param", Value::Integer(0))])
        );

        Ok(())
    }

    #[test]
    fn test_line_parameter() -> Test {
        assert_eq!(
            parse!("param=0"),
            Line::Parameter("param", Value::Integer(0))
        );
        assert_eq!(
            parse!("path/to/param=0"),
            Line::Parameter("path/to/param", Value::Integer(0))
        );

        Ok(())
    }

    #[test]
    fn test_value_literals() -> Test {
        assert_eq!(parse_val!("null"), Value::Null);
        assert_eq!(parse_val!("true"), Value::Boolean(true));
        assert_eq!(parse_val!("false"), Value::Boolean(false));
        assert_eq!(parse_val!("01234"), Value::Integer(1234));
        assert_eq!(parse_val!("-0123"), Value::Integer(-123));
        assert_eq!(parse_val!("1.0"), Value::Float(1.0));
        assert_eq!(parse_val!("-1.0"), Value::Float(-1.0));
        assert_eq!(parse_val!("\"ok\""), Value::String("ok"));

        Ok(())
    }

    #[test]
    fn test_value_array() -> Test {
        assert_eq!(
            parse_val!("[1, 2, 3]"),
            Value::Array(vec![
                Value::Integer(1),
                Value::Integer(2),
                Value::Integer(3)
            ])
        );

        Ok(())
    }

    #[test]
    fn test_value_map() -> Test {
        assert_eq!(
            parse_val!("{0:1, 2:3}"),
            Value::Map(vec![
                (Value::Integer(0), Value::Integer(1)),
                (Value::Integer(2), Value::Integer(3))
            ])
        );

        Ok(())
    }

    #[test]
    fn test_value_object() -> Test {
        assert_eq!(
            parse_val!(r#"Object(Object, "prop": 0)"#),
            Value::Object("Object", vec![("prop", Value::Integer(0))])
        );

        Ok(())
    }

    #[test]
    fn test_value_object_instance() -> Test {
        assert_eq!(
            parse_val!(r#"PackedByteArray(0,0,0)"#),
            Value::ObjectInstance(
                "PackedByteArray",
                vec![Value::Integer(0), Value::Integer(0), Value::Integer(0)],
            )
        );

        Ok(())
    }
}
