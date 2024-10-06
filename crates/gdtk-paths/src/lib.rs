#![feature(decl_macro)]

use std::{env::VarError as EnvVarError, io::Error as IOError};

pub use camino;
use camino::{Utf8Path, Utf8PathBuf};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("I/O error: {0:?}")]
    IOError(#[from] IOError),
    #[error("Environment variable error: {0:?}")]
    EnvVarError(#[from] EnvVarError),
    #[error("Camino error: {0:?}")]
    CaminoPathError(#[from] camino::FromPathError),
    #[error("Camino error: {0:?}")]
    CaminoPathBufError(#[from] camino::FromPathBufError),
}

/// A `bool` newtype to make the usage of [`ensure_dir_exists`] (and specifically it's
/// output) more readable.
#[must_use]
pub struct EnsureDirExistsOutput(bool);

impl EnsureDirExistsOutput {
    /// Did the directory already exist?
    pub fn already_existed(self) -> bool {
        self.0
    }
}

/// Attempts to create a folder at the given path and returns whether it succeded in doing so
/// (i.e. whether the folder already existed).
pub fn ensure_dir_exists(path: &Utf8Path) -> Result<EnsureDirExistsOutput, Error> {
    if path.exists() {
        return Ok(EnsureDirExistsOutput(true));
    }

    std::fs::create_dir(path)?;

    Ok(EnsureDirExistsOutput(false))
}

macro dirs_wrapper($(#[$attr:meta])* $public:vis $base:ident) {
    $(#[$attr])*
    $public fn $base() -> ::std::result::Result<$crate::camino::Utf8PathBuf, $crate::Error> {
        Ok(::dirs::$base()
            .ok_or(::std::io::Error::from(::std::io::ErrorKind::NotFound))
            .map($crate::camino::Utf8PathBuf::try_from)??)
    }
}

/// Create a function that returns a path. Example usage:
///
/// ```rs
/// dir! {
///     /// Returns the path to `~/.local/bin`.
///     pub local_bin_path: { Utf8PathBuf::from(std::env::var("HOME")?) }
///     / #[dir: true] ".local"
///     / #[dir: true] "bin"
/// }
/// ```
///
/// This will create a `pub fn local_bin_path` that returns a path to `~/.local/bin`.
/// The block after the colon is called a path's *base*, and can be either a full block
/// expression that returns a [`camino::Utf8PathBuf`] or an identifier that refers to
/// a function with an equivalent body (e.g. another `dir!`-defined function).
///
/// Every subsequent path segment must be a string literal annotated with a `#[dir: <bool>]`,
/// where `<bool>` is either `true` or `false`. If it's `true`, the generated function will
/// automatically create a directory at the given path using [`ensure_dir_exists`]. If it's
/// `false`, the generated function will not handle that path in any way. Generally, path
/// segments annotated with `#[dir: false]` are files are therefore should be the last path
/// segment. Putting a file segment anywhere else in the chain is undefined behaviour and
/// will likely result in I/O errors at runtime.
pub macro dir {
    // the entrypoint. accepts possible documentation/atttibutes and the identifier, then hands
    // the inner body handling to other (private) macro arms
    (
        $(#[$attr:meta])*
        $public:vis
        $name:ident:

        $($body:tt)+
    ) => {
        $(#[$attr])*
        $public fn $name() -> ::std::result::Result<$crate::camino::Utf8PathBuf, $crate::Error> {
            $crate::dir!(@priv $($body)+)
        }
    },

    // a shortcut to remove boilerplate when no special code is required to retrieve the base path
    (
        @priv

        $base:ident

        $($body:tt)*
    ) => {
        $crate::dir!(
            @priv

            { $base()? }
            $($body)+
        )
    },

    // the additions to the base path
    (
        @priv

        $base:block
        $(/ #[dir: $is_dir:literal] $new:literal)*
    ) => {
        {
            #[allow(unused_mut)]
            let mut base = $base;

            $({
                base.push($new);

                if $is_dir {
                    let _ = $crate::ensure_dir_exists(&base)?;
                }
            })*

            ::std::result::Result::Ok(base)
        }
    },
}

dirs_wrapper!(pub config_local_dir);
dirs_wrapper!(pub data_local_dir);
dirs_wrapper!(pub home_dir);

dir! {
    /// TODO
    pub base_conf_dir: config_local_dir
    / #[dir: true] "gdtk"
}

dir! {
    /// TODO
    pub base_data_dir: data_local_dir
    / #[dir: true] "gdtk"
}

dir! {
    /// TODO
    pub godots_path: base_data_dir
    / #[dir: true] "godots"
}

dir! {
    /// TODO
    pub logs_path: base_data_dir
    / #[dir: true] "logs"
}

pub fn executable_path() -> Result<Utf8PathBuf, Error> {
    dir! {
        gdtk_bin_dir: {
            Utf8PathBuf::from(std::env::var("GDTK_BIN_DIR")?)
        }
    }

    dir! {
        xdg_bin_home: {
            Utf8PathBuf::from(std::env::var("XDG_BIN_HOME")?)
        }
    }

    dir! {
        xdg_data_home: {
            Utf8PathBuf::from(std::env::var("XDG_DATA_HOME")?)
        }
        / #[dir: true] ".."
        / #[dir: true] "bin"
    }

    dir! {
        local_bin_path: home_dir
        / #[dir: true] ".local"
        / #[dir: true] "bin"
    }

    gdtk_bin_dir()
        .or(xdg_bin_home())
        .or(xdg_data_home())
        .or(local_bin_path())
}

/// Returns the path to the default godot executable.
pub fn default_godot_path() -> Result<Utf8PathBuf, Error> {
    let mut base = executable_path()?;

    let suffix = if cfg!(windows) { ".lnk" } else { "" };
    base.push(format!("godot{}", suffix));

    Ok(base)
}

dir! {
    pub local_versions_path: base_data_dir
    / #[dir: false] "local_versions"
}
