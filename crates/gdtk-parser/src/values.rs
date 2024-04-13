
use gdtk_ast::poor::{ASTValue, DictValue};
use gdtk_lexer::{Token, TokenKind};

use crate::{
    expressions::parse_expr,
    utils::{delemited_by, expect}, Parser,
};

pub fn parse_array<'a>(parser: &mut Parser<impl Iterator<Item = Token<'a>>>) -> Vec<ASTValue<'a>> {
    parser.next();

    let value = delemited_by(
        parser,
        TokenKind::Comma,
        &[TokenKind::ClosingBracket],
        parse_expr,
    );

    expect!(parser, TokenKind::ClosingBracket);

    value
}

pub fn parse_dictionary<'a>(parser: &mut Parser<impl Iterator<Item = Token<'a>>>) -> DictValue<'a> {
    expect!(parser, TokenKind::OpeningBrace);

    let value = match parser.peek().expect("unexpected EOF").kind {
        TokenKind::ClosingBrace => vec![], // empty dict
        TokenKind::Identifier(_) => parse_lua_dict(parser),
        _ => parse_python_dict(parser),
    };

    expect!(parser, TokenKind::ClosingBrace);

    value
}

/// Parse a lua-style dictionary body.
fn parse_lua_dict<'a>(parser: &mut Parser<impl Iterator<Item = Token<'a>>>) -> DictValue<'a> {
    fn parse_lua_key_value<'a>(
        parser: &mut Parser<impl Iterator<Item = Token<'a>>>,
    ) -> (ASTValue<'a>, ASTValue<'a>) {
        let key = ASTValue::Identifier(expect!(parser, TokenKind::Identifier(s), s));
        expect!(parser, TokenKind::Assignment);
        let value = parse_expr(parser);

        (key, value)
    }

    delemited_by(
        parser,
        TokenKind::Comma,
        &[TokenKind::ClosingBrace],
        parse_lua_key_value,
    )
}

/// Parse a python-style dictionary body.
fn parse_python_dict<'a>(parser: &mut Parser<impl Iterator<Item = Token<'a>>>) -> DictValue<'a> {
    fn parse_python_key_value<'a>(
        parser: &mut Parser<impl Iterator<Item = Token<'a>>>,
    ) -> (ASTValue<'a>, ASTValue<'a>) {
        let key = parse_expr(parser);
        expect!(parser, TokenKind::Colon);
        let value = parse_expr(parser);

        (key, value)
    }

    delemited_by(
        parser,
        TokenKind::Comma,
        &[TokenKind::ClosingBrace],
        parse_python_key_value,
    )
}

#[cfg(test)]
mod tests {
    use gdtk_ast::poor::*;

    use crate::test_utils::create_parser;
    use crate::values::{parse_array, parse_dictionary};

    #[test]
    fn test_parse_empty_array() {
        let mut parser = create_parser("[]");
        let result = parse_array(&mut parser);
        let expected = vec![];

        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_array() {
        let mut parser = create_parser("[1, 2, 3]");
        let result = parse_array(&mut parser);
        let expected = vec![
            ASTValue::Number(1),
            ASTValue::Number(2),
            ASTValue::Number(3),
        ];

        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_empty_dictionary() {
        let mut parser = create_parser("{}");
        let result = parse_dictionary(&mut parser);
        let expected = vec![];

        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_python_dictionary() {
        let mut parser = create_parser("{'a': 1, 'b': 2}");
        let result = parse_dictionary(&mut parser);
        let expected = vec![
            (ASTValue::String("a"), ASTValue::Number(1)),
            (ASTValue::String("b"), ASTValue::Number(2)),
        ];

        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_lua_dictionary() {
        let mut parser = create_parser("{a = 1, b = 2}");
        let result = parse_dictionary(&mut parser);
        let expected = vec![
            (ASTValue::Identifier("a"), ASTValue::Number(1)),
            (ASTValue::Identifier("b"), ASTValue::Number(2)),
        ];

        assert_eq!(result, expected);
    }
}
