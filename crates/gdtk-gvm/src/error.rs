use std::io::Error as IOError;

use gdtk_paths::Error as GdtkPathsError;
use rkyv::rancor::Error as RkyvError;
use ureq::Error as UreqError;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Unknown download URL: {0}.")]
    UnknownDownloadUrl(String),

    #[error("I/O error: {0:?}")]
    IOError(#[from] IOError),

    #[error("Rkyv error: {0:?}")]
    RkyvError(#[from] RkyvError),

    #[error("Surf error: {0:?}")]
    UreqError(#[from] UreqError),

    #[error("gdtk-paths error: {0:?}")]
    GdtkPathsError(#[from] GdtkPathsError),

    #[error("Unable to retrieve GitHub authentication token. Is `gh` set up on your machine?")]
    TokenRetrievalError,
}
