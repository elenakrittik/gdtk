use std::iter::Peekable;

use crate::error::Error;

#[doc(hidden)]
/// An trait version of [std::iter::Peekable]. Note that this is an implementation detail
/// made `pub` because of it's use in `crate::Parser`'s `I` type parameter in the
/// `crate::parser`/`crate::lexer` functions. Use at your own risk.
pub trait PeekableIterator: Iterator {
    fn peek(&mut self) -> Option<&Self::Item>;
}

#[doc(hidden)]
/// An extension trait that adds `*_ok` methods to `PeekableIterarator`s. Note that this
/// is an implementation detail made `pub` because of it's use in `crate::Parser`'s `I`
/// type parameter in the `crate::parser`/`crate::lexer` functions. Use at your own risk.
pub trait ResultIterator: Iterator {
    fn next_ok(&mut self) -> Result<Self::Item, Error<'static>> {
        self.next().ok_or(Error::UnexpectedEof)
    }
}

/// A [ResultIterator] for [PeekableIterator]s. Note that this is an implementation
/// detail made `pub` because of it's use in `crate::Parser`'s `I` type parameter in
/// the `crate::parser`/`crate::lexer` functions. Use at your own risk.
#[doc(hidden)]
pub trait PeekableResultIterator: PeekableIterator + ResultIterator {
    fn peek_ok(&mut self) -> Result<&Self::Item, Error<'static>> {
        self.peek().ok_or(Error::UnexpectedEof)
    }
}

impl<I: Iterator> PeekableIterator for Peekable<I> {
    fn peek(&mut self) -> Option<&Self::Item> {
        self.peek()
    }
}

impl<I: Iterator> ResultIterator for I {}
impl<I: PeekableIterator + ResultIterator> PeekableResultIterator for I {}
