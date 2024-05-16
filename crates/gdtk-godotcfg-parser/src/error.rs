use crate::token::Token;

#[derive(Debug, thiserror::Error)]
pub enum Error<'a> {
    #[error("This config has `config_version = {0}`, which is not supported.")]
    UnsupportedCfgVersion(u8),
    #[error("Unexpected {0:?}, expected {1}.")]
    Unexpected(Token<'a>, &'a str),
}
