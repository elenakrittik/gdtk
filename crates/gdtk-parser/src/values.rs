use gdtk_ast::{ASTExpr, ASTFunction, DictValue};
use gdtk_lexer::{Token, TokenKind};

use crate::{
    expressions::parse_expr,
    functions::parse_func,
    utils::{delemited_by, expect},
    Parser,
};

pub fn parse_array<'a>(parser: &mut Parser<impl Iterator<Item = Token<'a>>>) -> Vec<ASTExpr<'a>> {
    parser.next();

    let value = parser.with_parens_ctx(true, |parser| {
        delemited_by(
            parser,
            TokenKind::Comma,
            &[TokenKind::ClosingBracket],
            parse_expr,
        )
    });

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
    ) -> (ASTExpr<'a>, ASTExpr<'a>) {
        let key = ASTExpr::Identifier(expect!(parser, TokenKind::Identifier(s), s));
        expect!(parser, TokenKind::Assignment);
        let value = parse_expr(parser);

        (key, value)
    }

    parser.with_parens_ctx(true, |parser| {
        delemited_by(
            parser,
            TokenKind::Comma,
            &[TokenKind::ClosingBrace],
            parse_lua_key_value,
        )
    })
}

/// Parse a python-style dictionary body.
fn parse_python_dict<'a>(parser: &mut Parser<impl Iterator<Item = Token<'a>>>) -> DictValue<'a> {
    fn parse_python_key_value<'a>(
        parser: &mut Parser<impl Iterator<Item = Token<'a>>>,
    ) -> (ASTExpr<'a>, ASTExpr<'a>) {
        let key = parse_expr(parser);
        expect!(parser, TokenKind::Colon);
        let value = parse_expr(parser);

        (key, value)
    }

    parser.with_parens_ctx(true, |parser| {
        delemited_by(
            parser,
            TokenKind::Comma,
            &[TokenKind::ClosingBrace],
            parse_python_key_value,
        )
    })
}

/// Parse a lambda function.
pub fn parse_lambda<'a>(parser: &mut Parser<impl Iterator<Item = Token<'a>>>) -> ASTFunction<'a> {
    parser.with_parens_ctx(false, |parser| {
        parse_func(parser, gdtk_ast::ASTFunctionKind::Regular, true)
    })
}

#[cfg(test)]
mod tests {
    use gdtk_ast::*;

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
        let expected = vec![ASTExpr::Number(1), ASTExpr::Number(2), ASTExpr::Number(3)];

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
            (ASTExpr::String("a"), ASTExpr::Number(1)),
            (ASTExpr::String("b"), ASTExpr::Number(2)),
        ];

        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_lua_dictionary() {
        let mut parser = create_parser("{a = 1, b = 2}");
        let result = parse_dictionary(&mut parser);
        let expected = vec![
            (ASTExpr::Identifier("a"), ASTExpr::Number(1)),
            (ASTExpr::Identifier("b"), ASTExpr::Number(2)),
        ];

        assert_eq!(result, expected);
    }
}
