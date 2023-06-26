use std::io::Read;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum SparsecError {
    #[error("EOF reached while trying to read.")]
    EndOfFile,

    #[error("Invalid argument(s) provided.")]
    BadArguments { reason: &'static str },

    #[error("Internal parser error. It's not your fault, please report this at https://github.com/elenakrittik/gdtk/issues/")]
    InternalParserError { details: String },
}

pub struct Sparsec<'a, R: Read> {
    pub stream: &'a mut R,
}

impl<'a, R: Read> Sparsec<'a, R> {
    /// Returns
    pub fn new(stream: &'a mut R) -> Self {
        Self { stream }
    }

    pub fn read(&mut self, count: usize) -> Result<String, SparsecError> {
        let mut d = vec![0_u8];
        d.resize(count, 0);
        let buf = d.as_mut_slice();

        self.stream
            .read_exact(buf)
            .map_err(|e| SparsecError::InternalParserError {
                details: e.to_string(),
            })?;

        utf8_string(buf)
    }

    pub fn read_until(&mut self, until: &str) -> Result<String, SparsecError> {
        if until.chars().count() < 1 {
            return Ok("".to_string());
        }

        let mut buf = [0_u8];
        let mut utf8: Vec<u8> = Vec::new();
        let hint = until.chars().last().unwrap() as u8;

        while self.stream.read_exact(&mut buf).is_ok() {
            let byte = *buf.first().unwrap();
            utf8.push(byte);

            if byte == hint {
                let s = utf8_string(&utf8)?;
                let split = s.split(until).collect::<Vec<&str>>();

                if split.len() >= 2 {
                    utf8 = utf8[..utf8.len() - 1].to_vec();
                    break;
                }
            }
        }

        utf8_string(&utf8)
    }

    /// Read all of the remaining stream data.
    ///
    /// # Examples
    ///
    /// ```
    /// use fractparse::Sparsec;
    ///
    /// let mut stream = "AliceWoodhood".as_bytes();
    /// let mut sparsec = Sparsec::new(&mut stream);
    ///
    /// sparsec.read(5).unwrap();
    /// assert_eq!("Woodhood", sparsec.read_remaining().unwrap());
    /// ```
    pub fn read_remaining(&mut self) -> Result<String, SparsecError> {
        let mut s = String::new();
        self.stream
            .read_to_string(&mut s)
            .map_err(|e| SparsecError::InternalParserError {
                details: e.to_string(),
            })?;
        Ok(s)
    }
}

pub fn utf8_string(utf8: &[u8]) -> Result<String, SparsecError> {
    String::from_utf8(utf8.to_vec()).map_err(|e| SparsecError::InternalParserError {
        details: e.to_string(),
    })
}

#[cfg(test)]
mod tests {
    use crate::Sparsec;

    #[test]
    fn test_read_one() {
        let mut stream = "10".as_bytes();
        let mut parser = Sparsec::new(&mut stream);

        assert_eq!("1", parser.read(1).unwrap());
        assert_eq!("0", parser.read_remaining().unwrap());
    }

    #[test]
    fn test_read_until_sep_present() {
        let mut stream = "100.02".as_bytes();
        let mut parser = Sparsec::new(&mut stream);

        assert_eq!("100", parser.read_until(".").unwrap());
        assert_eq!("02", parser.read_remaining().unwrap());
    }

    #[test]
    fn test_read_until_sep_not_present() {
        let mut stream = "100".as_bytes();
        let mut parser = Sparsec::new(&mut stream);

        assert_eq!("100", parser.read_until(".").unwrap());
        assert_eq!("", parser.read_remaining().unwrap());
    }

    #[test]
    fn test_read_until_sep_trailing() {
        let mut stream = "100.".as_bytes();
        let mut parser = Sparsec::new(&mut stream);

        assert_eq!("100", parser.read_until(".").unwrap());
        assert_eq!("", parser.read_remaining().unwrap());
    }

    #[test]
    fn test_read_until_sep_preceding() {
        let mut stream = ".02".as_bytes();
        let mut parser = Sparsec::new(&mut stream);

        assert_eq!("", parser.read_until(".").unwrap());
        assert_eq!("02", parser.read_remaining().unwrap());
    }
}
