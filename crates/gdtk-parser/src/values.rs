
use gdtk_ast::poor::{ASTValue, DictValue};
use gdtk_lexer::{Token, TokenKind};

use crate::{
    expressions::parse_expr,
    utils::{delemited_by, expect}, Parser,
};

pub fn parse_array<'a>(iter: &mut Parser<impl Iterator<Item = Token<'a>>>) -> Vec<ASTValue<'a>> {
    iter.next();

    let value = delemited_by(
        iter,
        TokenKind::Comma,
        &[TokenKind::ClosingBracket],
        parse_expr,
    );

    expect!(iter, TokenKind::ClosingBracket);

    value
}

pub fn parse_dictionary<'a>(iter: &mut Parser<impl Iterator<Item = Token<'a>>>) -> DictValue<'a> {
    expect!(iter, TokenKind::OpeningBrace);

    let value = match iter.peek().expect("unexpected EOF").kind {
        TokenKind::ClosingBrace => vec![], // empty dict
        TokenKind::Identifier(_) => parse_lua_dict(iter),
        _ => parse_python_dict(iter),
    };

    expect!(iter, TokenKind::ClosingBrace);

    value
}

/// Parse a lua-style dictionary body.
fn parse_lua_dict<'a>(iter: &mut Parser<impl Iterator<Item = Token<'a>>>) -> DictValue<'a> {
    fn parse_lua_key_value<'a>(
        iter: &mut Parser<impl Iterator<Item = Token<'a>>>,
    ) -> (ASTValue<'a>, ASTValue<'a>) {
        let key = ASTValue::Identifier(expect!(iter, TokenKind::Identifier(s), s));
        expect!(iter, TokenKind::Assignment);
        let value = parse_expr(iter);

        (key, value)
    }

    delemited_by(
        iter,
        TokenKind::Comma,
        &[TokenKind::ClosingBrace],
        parse_lua_key_value,
    )
}

/// Parse a python-style dictionary body.
fn parse_python_dict<'a>(iter: &mut Parser<impl Iterator<Item = Token<'a>>>) -> DictValue<'a> {
    fn parse_python_key_value<'a>(
        iter: &mut Parser<impl Iterator<Item = Token<'a>>>,
    ) -> (ASTValue<'a>, ASTValue<'a>) {
        let key = parse_expr(iter);
        expect!(iter, TokenKind::Colon);
        let value = parse_expr(iter);

        (key, value)
    }

    delemited_by(
        iter,
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
