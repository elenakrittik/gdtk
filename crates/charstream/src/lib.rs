#[derive(thiserror::Error, Debug)]
pub enum CharStreamError {
    #[error("End-Of-Input reached.")]
    EndOfInput,

    #[error("Start-Of-Input reached.")]
    StartOfInput,

    #[error("Cursor position was out of stream's bounds.")]
    OutOfBoundsAccess,
}

#[derive(Debug, Clone)]
pub struct CharStream {
    inner: Vec<char>,
    pos: usize,
    len: usize,
}

impl CharStream {
    pub fn new(content: &String) -> CharStream {
        let chrs: Vec<char> = content.chars().collect();
        CharStream { pos: 0, len: chrs.len(), inner: chrs }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn remaining<'a>(&mut self) -> Result<Vec<char>, CharStreamError> {
        let mut chars = Vec::new();

        while let Ok(c) = self.next() {
            chars.push(c)
        }

        Ok(chars)
    }

    pub fn get(&self) -> Result<char, CharStreamError> {
        self.inner.get(self.pos).ok_or(CharStreamError::OutOfBoundsAccess).map(|v| *v)
    }

    fn safe_inc(&mut self, count: usize) -> Result<(), CharStreamError> {
        self.pos = self.pos.checked_add(count).ok_or(CharStreamError::EndOfInput)?;

        if self.pos >= self.len {
            self.pos = self.len - 1; // reset back to the end
            return Err(CharStreamError::EndOfInput);
        }

        Ok(())
    }

    fn safe_dec(&mut self, count: usize) -> Result<(), CharStreamError> {
        self.pos = self.pos.checked_sub(count).ok_or(CharStreamError::StartOfInput)?;

        // usize is always positive so no need to check against pos < len here
        // (because len >= 0)
    
        Ok(())
    }

    pub fn next(&mut self) -> Result<char, CharStreamError> {
        self.safe_inc(1)?;
        self.get()
    }

    pub fn prev(&mut self) -> Result<char, CharStreamError> {
        self.safe_dec(1)?;
        self.get()
    }

    pub fn goto(&mut self, pos: usize) -> Result<(), CharStreamError> {
        let diff = pos as i64 - self.pos as i64;

        if diff > 0 {
            self.safe_inc(diff as usize)
        } else if diff < 0 {
            self.safe_dec(diff.abs() as usize)
        } else {
            Ok(())
        }
    }
}
