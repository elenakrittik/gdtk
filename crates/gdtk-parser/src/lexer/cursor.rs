type PeekableCharIndices<'a> = std::iter::Peekable<std::str::CharIndices<'a>>;

pub(super) struct Cursor<'a> {
    current_token_position: usize,
    chars: PeekableCharIndices<'a>,
}

impl<'a> Cursor<'a> {
    pub(super) fn new(source: &'a str) -> Self {
        Self {
            current_token_position: 0,
            chars: source.char_indices().peekable(),
        }
    }

    pub(super) fn peek(&mut self) -> Option<&char> {
        self.chars.peek().map(|(_, c)| c)
    }

    pub(super) fn start_token(&mut self) {
        self.current_token_position = self
            .chars
            .peek()
            .map(|(pos, _)| *pos)
            .unwrap_or_else(|| self.chars.clone().count());
    }

    pub(super) fn current_span(&mut self) -> gdtk_span::Span {
        self.current_token_position
            ..self
                .chars
                .peek()
                .map(|(pos, _)| *pos)
                .unwrap_or_else(|| self.chars.clone().count())
    }
}

impl<'a> Iterator for Cursor<'a> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        self.chars.next().map(|(_, c)| c)
    }
}
