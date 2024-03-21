use std::io::Error as IOError;

use reqwest::Error as ReqError;
use toml::de::Error as TOMLDeError;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("I/O error: {0:?}")]
    IOError(IOError),

    #[error("TOML deserialization error: {0:?}")]
    TOMLDeserializationError(TOMLDeError),

    #[error("Reqwest error: {0:?}")]
    ReqwestError(ReqError),
}

impl From<IOError> for Error {
    fn from(value: IOError) -> Self {
        Self::IOError(value)
    }
}

impl From<TOMLDeError> for Error {
    fn from(value: TOMLDeError) -> Self {
        Self::TOMLDeserializationError(value)
    }
}

impl From<ReqError> for Error {
    fn from(value: ReqError) -> Self {
        Self::ReqwestError(value)
    }
}
