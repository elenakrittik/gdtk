pub mod rustc;

use crate::diagnostic::Span;

/// Helper struct for querying information in sources.
pub struct Source<'a> {
    source: &'a str,
}

impl<'a> Source<'a> {
    /// Create a new [Source].
    pub fn new(source: &'a str) -> Self {
        Self { source }
    }

    /// Gives a human-friendly representation of a span: `(line, column)`.
    pub fn locate(&self, span: &Span) -> (usize, usize) {
        let mut line = 0usize;
        let mut column = 0usize;

        let mut chars = self.source.chars().enumerate();

        loop {
            let Some((idx, c)) = chars.next() else {
                break;
            };

            column += 1;

            match c {
                '\n' => {
                    line += 1;
                    column = 0;
                }
                _ => (),
            }

            if span.contains(&idx) {
                break;
            }
        }

        (line, column)
    }

    /// Get the `n`th line in the source code.
    pub fn line(&self, n: usize) -> Option<&'a str> {
        self.source.split('\n').nth(n)
    }
}
