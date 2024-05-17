use std::iter::Peekable;

pub trait PeekableIterator: Iterator {
    fn peek(&mut self) -> Option<&Self::Item>;
    fn peek_mut(&mut self) -> Option<&mut Self::Item>;
}

impl<I: Iterator> PeekableIterator for Peekable<I> {
    fn peek(&mut self) -> Option<&Self::Item> {
        self.peek()
    }

    fn peek_mut(&mut self) -> Option<&mut Self::Item> {
        self.peek_mut()
    }
}
