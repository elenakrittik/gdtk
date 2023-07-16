//! `charstream` - a convenient wrapper around `std`'s `String` that provides
//! a two-way stream-like APIs. Perfect for parser libraries.

#[derive(thiserror::Error, Debug)]
pub enum CharStreamError {
    #[error("End-Of-Input reached.")]
    EndOfInput,

    #[error("Cursor position was out of stream's bounds. Please report this at https://github.com/elenakrittik/gdtk/issues")]
    OutOfBoundsAccess,

    #[error("Start-Of-Input reached.")]
    StartOfInput,
}

#[derive(Debug, Clone)]
pub struct CharStream<'a> {
    original: &'a String,
    inner: Vec<char>,
    pos: usize,
    len: usize,
}

impl<'a> CharStream<'a> {
    /// Create a new [CharStream].
    pub fn new(content: &String) -> CharStream {
        let chrs: Vec<char> = content.chars().collect();
        CharStream {
            pos: 0,
            len: chrs.len(),
            inner: chrs,
            original: content,
        }
    }

    /// Returns the total length of the stream.
    #[allow(clippy::len_without_is_empty)]
    pub fn len(&self) -> usize {
        self.len
    }

    /// Reads the stream until the end and returns all characters read.
    pub fn remaining(&mut self) -> &str {
        let original = self.pos.clone();
        self.goto(self.len - 1).unwrap();
        &self.original[original..]
    }

    /// Gets current character.
    pub fn get(&self) -> Result<char, CharStreamError> {
        self.inner
            .get(self.pos)
            .ok_or(CharStreamError::OutOfBoundsAccess)
            .map(|v| *v)
    }

    fn safe_inc(&mut self, count: usize) -> Result<(), CharStreamError> {
        self.pos = self
            .pos
            .checked_add(count)
            .ok_or(CharStreamError::EndOfInput)?;

        if self.pos >= self.len {
            self.pos = self.len - 1; // reset back to the end
            return Err(CharStreamError::EndOfInput);
        }

        Ok(())
    }

    fn safe_dec(&mut self, count: usize) -> Result<(), CharStreamError> {
        self.pos = self
            .pos
            .checked_sub(count)
            .ok_or(CharStreamError::StartOfInput)?;

        // usize is always positive so no need to check against pos < len here
        // (because len >= 0)

        Ok(())
    }

    /// Moves cursor one time forward and returns the result of [CharStream::get].
    #[allow(clippy::should_implement_trait)]
    pub fn next(&mut self) -> Result<char, CharStreamError> {
        self.safe_inc(1)?;
        self.get()
    }

    /// Moves cursor one time backward and returns the result of [CharStream::get].
    pub fn prev(&mut self) -> Result<char, CharStreamError> {
        self.safe_dec(1)?;
        self.get()
    }

    /// Same as [CharStream::goto], but also returns slice implied by the transition.
    pub fn travel(&mut self, pos: usize) -> Result<&str, CharStreamError> {
        let origin = self.pos.clone();
        self.goto(pos)?;
        Ok(&self.original[origin..self.pos])
    }

    /// Immediately moves cursor to the specified position.
    pub fn goto(&mut self, pos: usize) -> Result<(), CharStreamError> {
        let diff = (pos as isize) - (self.pos as isize);

        self.mov(diff)
    }

    /// Move cursor `diff` times forward or backward, depending on whether
    /// `diff` is positive or negative.
    pub fn mov(&mut self, diff: isize) -> Result<(), CharStreamError> {
        #[allow(clippy::comparison_chain)]
        if diff > 0 {
            self.safe_inc(diff as usize)
        } else if diff < 0 {
            self.safe_dec(diff.unsigned_abs() as usize)
        } else {
            Ok(())
        }
    }
}
