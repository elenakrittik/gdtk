use std::iter::Peekable;

use crate::error::Error;

#[doc(hidden)]
/// An trait version of [std::iter::Peekable]. Note that this is an implementation detail
/// made `pub` because of it's use in `crate::Parser`'s `I` type parameter in the
/// `crate::parser`/`crate::lexer` functions. Use at your own risk.
pub trait PeekableIterarator: Iterator {
    fn peek(&mut self) -> Option<&Self::Item>;
    // fn peek_mut(&mut self) -> Option<&mut Self::Item>;
}

impl<I: Iterator> PeekableIterarator for Peekable<I> {
    fn peek(&mut self) -> Option<&Self::Item> {
        self.peek()
    }

    // fn peek_mut(&mut self) -> Option<&mut Self::Item> {
    //     self.peek_mut()
    // }
}

#[doc(hidden)]
/// An extension trait that adds `*_ok` methods to `PeekableIterarator`s. Note that this
/// is an implementation detail made `pub` because of it's use in `crate::Parser`'s `I`
/// type parameter in the `crate::parser`/`crate::lexer` functions. Use at your own risk.
pub trait ResultIterator: PeekableIterarator {
    fn next_ok(&mut self) -> Result<Self::Item, Error<'static>> {
        self.next().ok_or(Error::UnexpectedEof)
    }

    fn peek_ok(&mut self) -> Result<&Self::Item, Error<'static>> {
        self.peek().ok_or(Error::UnexpectedEof)
    }

    // fn peek_mut_ok(&mut self) -> Result<&mut Self::Item, Error<'static>> {
    //     self.peek_mut().ok_or(Error::UnexpectedEof)
    // }
}

impl<I: PeekableIterarator> ResultIterator for I {}
