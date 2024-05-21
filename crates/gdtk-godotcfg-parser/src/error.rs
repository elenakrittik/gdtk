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
}
