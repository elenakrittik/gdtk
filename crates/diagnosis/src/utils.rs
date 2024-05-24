/// Helper struct for querying information in sources.
pub struct Source<'a> {
    source: &'a str,
}

impl<'a> Source<'a> {
    /// Create a new [Source].
    pub fn new(source: &'a str) -> Self {
        Self { source }
    }

    /// Gives the first `(line, column)` that the given span contains (zero-indexed).
    /// Returns `None` if the span is out of source's bounds.
    pub fn locate(&self, span: &impl std::ops::RangeBounds<usize>) -> Option<(usize, usize)> {
        match span.end_bound() {
            std::ops::Bound::Included(end) if end > &self.source.len() => return None,
            std::ops::Bound::Excluded(end) if end >= &self.source.len() => return None,
            std::ops::Bound::Unbounded => return None,
            _ => (),
        }

        let mut line = 0usize;
        let mut column = 0usize;

        for (idx, c) in self.source.chars().enumerate() {
            column += 1;

            if c == '\n' {
                line += 1;
                column = 0;
            }

            if span.contains(&idx) {
                break;
            }
        }

        Some((line, column))
    }

    /// Get the `n`th (zero-indexed) line in the source code.
    pub fn line(&self, n: usize) -> Option<&'a str> {
        self.source.split('\n').nth(n)
    }
}
