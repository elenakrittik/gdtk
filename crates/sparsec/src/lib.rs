#![feature(decl_macro)]
#![feature(trait_alias)]

mod utils;
mod macros;

pub use charstream;
pub use crate::macros::*;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum SparsecError {
    #[error("EOF reached while trying to read.")]
    EndOfFile,

    #[error("End-of-input reached while trying to read.")]
    EndOfInput,

    #[error("Start-of-input reached while trying to read.")]
    StartOfInput,

    #[error("Invalid argument(s) provided.")]
    BadArguments { reason: &'static str },

    #[error("Internal parser error. It's not your fault, please report this at https://github.com/elenakrittik/gdtk/issues/")]
    InternalParserError { details: String },

    #[error("All choice subparsers failed.")]
    ChoiceFailed,

    #[error("Unexpected character encountered.")]
    UnexpectedCharacter {
        expected: char,
        encountered: char,
    },
}

pub type Stream = charstream::CharStream;

#[derive(Debug, Clone)]
pub struct Sparsec {
    pub stream: Stream,
}

type ChoiceFnType<T, E> = fn(&mut Sparsec) -> Result<T, E>;

impl Sparsec {
    /// Returns
    pub fn new(stream: Stream) -> Self {
        Self {
            stream,
        }
    }

    /// Read `count` characters and concatenate into a [String].
    pub fn read_string(&mut self, count: usize) -> Result<String, SparsecError> {
        Ok(self.read(count)?.iter().collect())
    }

    /// Read `count` characters.
    pub fn read(&mut self, count: usize) -> Result<Vec<char>, SparsecError> {
        let mut s = Vec::new();

        for _ in 0..count {
            s.push(self.read_one()?)
        }

        Ok(s)
    }

    /// Optimized version of [read] for reading a single character.
    pub fn read_one(&mut self) -> Result<char, SparsecError> {
        self.stream.next().map_err(map_charstream_error)
    }

    pub fn read_one_exact(&mut self, expected: &char) -> Result<char, SparsecError> {
        let chr = self.read_one()?;

        if chr != *expected {
            return Err(SparsecError::UnexpectedCharacter { expected: *expected, encountered: chr });
        }

        Ok(chr)
    }

    pub fn read_until(&mut self, until: &str) -> Result<Vec<char>, SparsecError> {
        if until.chars().count() < 1 {
            return Ok(vec![]);
        }

        let mut chars: Vec<char> = Vec::new();
        let hint = until.chars().last().unwrap();

        while let Ok(chr) = self.read_one() {
            chars.push(chr);

            if chr == hint {
                let s = chars.clone().into_iter().collect::<String>();
                let split = s.split(until).collect::<Vec<&str>>();

                if split.len() >= 2 {
                    chars = chars[..chars.len() - 1].to_vec();
                    break;
                }
            }
        }

        Ok(chars)
    }

    /// Consume input as long as `pred(character)` returns `true`.
    /// 
    /// Clone count: 1
    pub fn read_while(&mut self, pred: fn(&char) -> bool) -> Result<Vec<char>, SparsecError> {
        let mut result = Vec::new();
        let mut safe = self.clone();
        let mut num_read = 0;

        while let Ok(v) = safe.read_one() {
            if pred(&v) {
                result.push(v);
                num_read += 1;
            } else {
                break;
            }
        }

        self.read(num_read)?;

        Ok(result)
    }

    /// Read all of the remaining stream data.
    ///
    /// # Examples
    ///
    /// ```
    /// sparsec::from_string!(parser, "AliceWoodhood");
    ///
    /// parser.read(5).unwrap();
    /// assert_eq!("Woodhood", parser.read_remaining().unwrap());
    /// ```
    pub fn read_remaining(&mut self) -> Result<Vec<char>, SparsecError> {
        self.stream.remaining().map_err(map_charstream_error)
    }

    pub fn choice<T, E>(&mut self, fns: Vec<ChoiceFnType<T, E>>) -> Result<T, SparsecError> {
        for func in fns {
            if let Ok(val) = func(&mut self.clone()) {
                return Ok(val);
            }
        }

        Err(SparsecError::ChoiceFailed)
    }
}

/// Helper macro for creating a [Sparsec] instance from a [String].
///
/// # Examples
///
/// ```
/// sparsec::from_string!(parser, "10");
///
/// assert_eq!("10", parser.read_string(2).unwrap())
/// ```
pub macro from_string($var: ident, $s: expr) {
    let mut $var = $crate::Sparsec::new($crate::charstream::CharStream::new(&$s.to_string()));
}

fn map_charstream_error(e: charstream::CharStreamError) -> SparsecError {
    match e {
        charstream::CharStreamError::EndOfInput => SparsecError::EndOfInput,
        charstream::CharStreamError::StartOfInput => SparsecError::StartOfInput,
        _ => SparsecError::InternalParserError { details: e.to_string() },
    }
}

// #[cfg(test)]
// mod tests {
//     use crate::Sparsec;

//     #[test]
//     fn test_choice_double() {
//         crate::from_string!(parser, "kate");

//         fn lena(parser: &mut Sparsec) -> Result<String, ()> {
//             let s = parser.read_string(4).map_err(|e| println!("{:?}", e))?;
//             if s == "lena" {
//                 Ok(s)
//             } else {
//                 Err(())
//             }
//         }

//         fn kate(parser: &mut Sparsec) -> Result<String, ()> {
//             let s = parser.read_string(4).map_err(|e| println!("{:?}", e))?;
//             if s == "kate" {
//                 Ok(s)
//             } else {
//                 Err(())
//             }
//         }

//         assert_eq!("kate", parser.choice(vec![lena, kate]).unwrap());
//     }

//     #[test]
//     fn test_read_while() {
//         crate::from_string!(parser, "1000abc");

//         assert_eq!("1000", parser.read_while(|c| char::is_ascii_digit(c)).unwrap().into_iter().collect());
//         assert_eq!("abc", parser.read_remaining().unwrap().into_iter().collect());
//     }

//     #[test]
//     fn test_read_one() {
//         crate::from_string!(parser, "10");

//         assert_eq!('1', parser.read_one().unwrap());
//         assert_eq!("0", parser.read_remaining().unwrap());
//     }

//     #[test]
//     fn test_read_until_sep_present() {
//         crate::from_string!(parser, "100.02");

//         assert_eq!("100", parser.read_until(".").unwrap());
//         assert_eq!("02", parser.read_remaining().unwrap());
//     }

//     #[test]
//     fn test_read_until_sep_not_present() {
//         crate::from_string!(parser, "100");

//         assert_eq!("100", parser.read_until(".").unwrap());
//         assert_eq!("", parser.read_remaining().unwrap());
//     }

//     #[test]
//     fn test_read_until_sep_trailing() {
//         crate::from_string!(parser, "100.");

//         assert_eq!("100", parser.read_until(".").unwrap());
//         assert_eq!("", parser.read_remaining().unwrap());
//     }

//     #[test]
//     fn test_read_until_sep_preceding() {
//         crate::from_string!(parser, ".02");

//         assert_eq!("", parser.read_until(".").unwrap());
//         assert_eq!("02", parser.read_remaining().unwrap());
//     }
// }
