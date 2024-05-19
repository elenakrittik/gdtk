use crate::token::Token;

#[derive(Default, Debug, Clone, PartialEq, thiserror::Error)]
pub enum Error<'a> {
    #[default]
    #[error("Unrecognised token.")]
    UnrecognisedToken,
    #[error("This config has `config_version = {0}`, which is not supported.")]
    UnsupportedCfgVersion(u8),
    #[error("Unexpected {0:?}, expected {1}.")]
    Unexpected(Token<'a>, &'a str),
    #[error("Unexpected EOF.")]
    UnexpectedEof,
    #[error("Bytes in `PackedByteArray`s must fit in a `u8`.")]
    ByteDoesntFit(Token<'a>),
    #[error("Unrecognised object: {0:?}.")]
    UnrecognisedObject(&'a str),
}
