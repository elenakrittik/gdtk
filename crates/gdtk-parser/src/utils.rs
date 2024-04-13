
use gdtk_lexer::{Token, TokenKind};

use crate::Parser;

/// Assert that the next token is of the given variant, and optionally
/// return it's value.
pub macro expect {
    ($iter:expr, $variant:pat) => {
        $crate::utils::expect!($iter, $variant, ())
    },
    ($iter:expr, $variant:pat, $ret:expr) => {{
        type Token<'a> = ::gdtk_lexer::Token<'a>;

        match $iter.next() {
            Some(Token { kind: $variant, .. }) => $ret,
            other => panic!("expected {}, found {other:?}", stringify!($variant)),
        }
    }}
}

/// Parses a list of values (as defined by the passed callback) separated by the specified delimiter.
/// ``stop_at`` is used to know when to stop looking for new values.
pub fn delemited_by<'a, I, V>(
    parser: &mut Parser<I>,
    delimiter: TokenKind<'a>,
    stop_at: &[TokenKind<'a>],
    mut callback: impl FnMut(&mut Parser<I>) -> V,
) -> Vec<V>
where
    I: Iterator<Item = Token<'a>>,
{
    let mut values = vec![];

    while parser
        .peek()
        .is_some_and(|t| !(stop_at.iter().any(|k| k.same_as(&t.kind))))
    {
        values.push(callback(parser));

        if parser.peek().is_some_and(|t| t.kind.same_as(&delimiter)) {
            parser.next();
        }
    }

    values
}

/// Calls ``iter.next()``, then ``callback(iter)``.
pub fn advance_and_parse<'a, I, V>(
    parser: &mut Parser<I>,
    mut callback: impl FnMut(&mut Parser<I>) -> V,
) -> V
where
    I: Iterator<Item = Token<'a>>,
{
    parser.next();
    callback(parser)
}

#[cfg(test)]
mod tests {
    use gdtk_lexer::TokenKind;

    use crate::test_utils::{create_parser, next_kind};
    use crate::utils::{advance_and_parse, delemited_by, expect};

    #[test]
    fn test_delemited_by() {
        let mut parser = create_parser("1, 2;");
        let result = delemited_by(
            &mut parser,
            TokenKind::Comma,
            &[TokenKind::Semicolon],
            next_kind,
        );
        let expected = vec![TokenKind::Integer(1), TokenKind::Integer(2)];

        assert_eq!(result, expected);
    }

    #[test]
    fn test_delemited_by_trailing() {
        let mut parser = create_parser("1, 2,;");
        let result = delemited_by(
            &mut parser,
            TokenKind::Comma,
            &[TokenKind::Semicolon],
            next_kind,
        );
        let expected = vec![TokenKind::Integer(1), TokenKind::Integer(2)];

        assert_eq!(result, expected);
    }

    #[test]
    fn test_delemited_by_single() {
        let mut parser = create_parser("1;");
        let result = delemited_by(
            &mut parser,
            TokenKind::Comma,
            &[TokenKind::Semicolon],
            next_kind,
        );
        let expected = vec![TokenKind::Integer(1)];

        assert_eq!(result, expected);
    }

    #[test]
    fn test_delemited_by_empty() {
        let mut parser = create_parser(";");
        let result = delemited_by(
            &mut parser,
            TokenKind::Comma,
            &[TokenKind::Semicolon],
            next_kind,
        );
        let expected = vec![];

        assert_eq!(result, expected);
    }

    #[test]
    fn test_advance_and_parse() {
        let mut parser = create_parser(".1");
        let result = advance_and_parse(&mut parser, next_kind);
        let expected = TokenKind::Integer(1);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_expect() {
        let mut parser = create_parser(";");
        expect!(&mut parser, TokenKind::Semicolon);
    }
}
