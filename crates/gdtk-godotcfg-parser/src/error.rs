use crate::token::Token;

#[derive(Default, Debug, Clone, PartialEq, thiserror::Error)]
pub enum Error<'a> {
    /// An unrecognised character sequence.
    #[default]
    #[error("Unrecognised token.")]
    UnrecognisedToken,

    /// An unexpected token.
    #[error("Unexpected {0:?}, expected {1}.")]
    Unexpected(Token<'a>, &'a str),

    /// Unexpected end-of-file.
    #[error("Unexpected EOF.")]
    UnexpectedEof,

    /// A byte integer in a `crate::ast::Value::PackedByteArray`
    /// does not fit in a `u8` (i.e., is not in range `0..256`).
    #[error("Bytes in `PackedByteArray`s must fit in a `u8`.")]
    ByteDoesntFit(Token<'a>),

    /// An unrecognised object value.
    #[error("Unrecognised object: {0:?}.")]
    UnrecognisedObject(&'a str),
}
