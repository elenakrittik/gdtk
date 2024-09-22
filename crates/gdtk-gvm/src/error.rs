use std::io::Error as IOError;

use toml::de::Error as TOMLDeError;
use toml::ser::Error as TOMLSerError;
use ureq::Error as UreqError;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Mono versions of Godot are not supported yet.")]
    MonoUnsupported,

    #[error("Unknown download URL: {0}.")]
    UnknownDownloadUrl(String),

    #[error("I/O error: {0:?}")]
    IOError(#[from] IOError),

    #[error("TOML serialization error: {0:?}")]
    TOMLSerializationError(#[from] TOMLSerError),

    #[error("TOML deserialization error: {0:?}")]
    TOMLDeserializationError(#[from] TOMLDeError),

    #[error("Surf error: {0:?}")]
    UreqError(#[from] UreqError),

    #[error("Unable to retrieve GitHub authentication token. Is `gh` set up on your machine?")]
    TokenRetrievalError,
}
