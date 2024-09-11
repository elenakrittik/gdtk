use std::io::Error as IOError;

use octocrab::Error as OctoError;
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
    IOError(#[from] IOError),

    #[error("TOML serialization error: {0:?}")]
    TOMLSerializationError(#[from] TOMLSerError),

    #[error("TOML deserialization error: {0:?}")]
    TOMLDeserializationError(#[from] TOMLDeError),

    #[error("Reqwest error: {0:?}")]
    ReqwestError(#[from] ReqError),

    #[error("GitHub API errorL {0:?}")]
    OctocrabError(#[from] OctoError),
}
