use std::iter::Peekable;

use crate::error::Error;

pub trait PeekableIterarator: Iterator {
    fn peek(&mut self) -> Option<&Self::Item>;
    fn peek_mut(&mut self) -> Option<&mut Self::Item>;
}

impl<I: Iterator> PeekableIterarator for Peekable<I> {
    fn peek(&mut self) -> Option<&Self::Item> {
        self.peek()
    }

    fn peek_mut(&mut self) -> Option<&mut Self::Item> {
        self.peek_mut()
    }
}

pub trait ResultIterator: PeekableIterarator {
    fn next_ok(&mut self) -> Result<Self::Item, Error<'static>> {
        self.next().ok_or(Error::UnexpectedEof)
    }

    fn peek_ok(&mut self) -> Result<&Self::Item, Error<'static>> {
        self.peek().ok_or(Error::UnexpectedEof)
    }

    fn peek_mut_ok(&mut self) -> Result<&mut Self::Item, Error<'static>> {
        self.peek_mut().ok_or(Error::UnexpectedEof)
    }
}

impl<I: PeekableIterarator> ResultIterator for I {}
