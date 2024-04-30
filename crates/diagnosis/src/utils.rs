use crate::Span;

/// Helper struct for querying information in sources.
pub struct Source<'a> {
    source: &'a str,
}

impl<'a> Source<'a> {
    /// Create a new [Source].
    pub fn new(source: &'a str) -> Self {
        Self { source }
    }

    /// Gives the first line that contains the given span (zero-indexed).
    /// Returns `None` if the span is out of source's bounds.
    pub fn locate(&self, span: &Span) -> Option<usize> {
        let mut line = 0usize;

        let mut chars = self.source.chars().enumerate();

        loop {
            let Some((idx, c)) = chars.next() else {
                break None;
            };

            if c == '\n' {
                line += 1;
            }

            if span.contains(&idx) {
                break Some(line);
            }
        }
    }

    /// Get the `n`th (zero-indexed) line in the source code.
    pub fn line(&self, n: usize) -> Option<&'a str> {
        self.source.split('\n').nth(n)
    }
}
