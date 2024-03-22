use std::io::Error as IOError;

use reqwest::Error as ReqError;
use toml::de::Error as TOMLDeError;
use toml::ser::Error as TOMLSerError;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Mono versions of Godot are not supported yet.")]
    MonoUnsupported,

    #[error("Unknown download URL: {0}.")]
    UnknownDownloadUrl(String),

    #[error("I/O error: {0:?}")]
    IOError(IOError),

    #[error("TOML serialization error: {0:?}")]
    TOMLSerializationError(TOMLSerError),

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

impl From<TOMLSerError> for Error {
    fn from(value: TOMLSerError) -> Self {
        Self::TOMLSerializationError(value)
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
