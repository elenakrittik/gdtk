use std::iter::Peekable;

use gdtk_lexer::{Token, TokenKind};

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
    iter: &mut Peekable<I>,
    delimiter: TokenKind<'a>,
    stop_at: &[TokenKind<'a>],
    mut callback: impl FnMut(&mut Peekable<I>) -> V,
) -> Vec<V>
where
    I: Iterator<Item = Token<'a>>,
{
    const BLANKETS: &[TokenKind<'static>] =
        &[TokenKind::Newline, TokenKind::Indent, TokenKind::Dedent];
    const PARENS: &[TokenKind<'static>] = &[
        TokenKind::ClosingParenthesis,
        TokenKind::ClosingBracket,
        TokenKind::ClosingBrace,
    ];

    let in_parens = stop_at
        .iter()
        .any(|k1| PARENS.iter().any(|k2| k2.same_as(k1)));
    let mut values = vec![];

    while iter
        .peek()
        .is_some_and(|t| !(stop_at.iter().any(|k| k.same_as(&t.kind))))
    {
        // ignore newlines and in-/de- dents inside parentheses
        if in_parens
            && iter
                .peek()
                .is_some_and(|t| BLANKETS.iter().any(|k| k.same_as(&t.kind)))
        {
            iter.next();
            continue;
        }

        values.push(callback(iter));

        if iter.peek().is_some_and(|t| t.kind.same_as(&delimiter)) {
            iter.next();
        }
    }

    values
}

/// Calls ``iter.next()``, then ``callback(iter)``.
pub fn advance_and_parse<'a, I, V>(
    iter: &mut Peekable<I>,
    mut callback: impl FnMut(&mut Peekable<I>) -> V,
) -> V
where
    I: Iterator<Item = Token<'a>>,
{
    iter.next();
    callback(iter)
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
    #[should_panic]
    fn test_advance_and_parse_invalid() {
        let mut parser = create_parser(";");
        advance_and_parse(&mut parser, next_kind);
    }

    #[test]
    fn test_expect() {
        let mut parser = create_parser(";");
        expect!(&mut parser, TokenKind::Semicolon);
    }
}
