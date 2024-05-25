use std::iter::Peekable;

use gdtk_span::Span;

use crate::lexer::Token;

/// A wrapper around token iterator with additional functionality.
#[derive(Debug)]
pub struct Parser<I> {
    pub iter: I,
    pub is_inside_parens: bool,
    pub current_token_span: Option<Span>,
}

impl<'a, I> Parser<Peekable<I>>
where
    I: Iterator<Item = Token<'a>>,
{
    pub fn new(iter: I) -> Self {
        Self {
            iter: iter.peekable(),
            is_inside_parens: false,
            current_token_span: None,
        }
    }

    pub fn span_start(&mut self) -> usize {
        self.peek().as_ref().map(|t| t.span.start).unwrap_or(0)
    }

    pub fn finish_span(&mut self, start: usize) -> Span {
        let end = self.current_token_span.as_ref().map(|r| r.end).unwrap_or(0);

        start..end
    }

    pub fn peek(&mut self) -> Option<&Token<'a>> {
        self.skip_blanks();
        self.iter.peek()
    }

    /// Invoke a function inside the context of a parenthesized expression.
    pub fn with_parens_ctx<F, R>(&mut self, val: bool, mut f: F) -> R
    where
        F: FnMut(&mut Self) -> R,
    {
        let previous = self.is_inside_parens;

        self.is_inside_parens = val;

        let result = f(self);

        self.is_inside_parens = previous;

        result
    }

    fn skip_blanks(&mut self) {
        if self.is_inside_parens {
            while self
                .iter
                .peek()
                .is_some_and(|t| t.kind.is_newline() || t.kind.is_indent() || t.kind.is_dedent())
            {
                self.iter.next();
            }
        }
    }
}

impl<'a, I> Iterator for Parser<Peekable<I>>
where
    I: Iterator<Item = Token<'a>>,
{
    type Item = Token<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.skip_blanks();
        self.current_token_span = self.iter.peek().map(|t| t.span.start..t.span.end);
        self.iter.next()
    }
}

#[cfg(test)]
mod tests {
    use crate::test_utils::create_parser;

    #[test]
    fn test_span_tracking() {
        let mut parser = create_parser("var hello = 2");

        assert!(parser.next().unwrap().kind.is_var());

        let start = parser.span_start();

        assert!(parser.next().unwrap().kind.is_identifier());
        assert!(parser.next().unwrap().kind.is_assignment());

        let span = parser.finish_span(start);

        assert_eq!(span, 4..11); // 'hello ='
    }
}
