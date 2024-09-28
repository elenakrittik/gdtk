#![feature(decl_macro)]

use std::io::Error as IOError;

pub use camino;
use camino::{Utf8Path, Utf8PathBuf};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("I/O error: {0:?}")]
    IOError(#[from] IOError),
    #[error("Camino error: {0:?}")]
    CaminoPathError(#[from] camino::FromPathError),
    #[error("Camino error: {0:?}")]
    CaminoPathBufError(#[from] camino::FromPathBufError),
}

pub fn ensure_path(path: &Utf8Path, dir: bool) -> Result<bool, Error> {
    if path.exists() {
        return Ok(false);
    }

    if dir {
        std::fs::create_dir(path)?;
    } else {
        std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(path)?;
    }

    Ok(true)
}

macro dirs_wrapper($(#[$attr:meta])* $base:ident) {
    $(#[$attr])*
    pub fn $base() -> ::std::result::Result<$crate::camino::Utf8PathBuf, $crate::Error> {
        Ok(::dirs::$base()
            .ok_or(::std::io::Error::from(::std::io::ErrorKind::NotFound))
            .map($crate::camino::Utf8PathBuf::try_from)??)
    }
}

pub macro dir {
    // the entrypoint. accepts possible documentation/atttibutes and the identifier, then hands
    // the inner body handling to other (private) macro arms
    (
        $(#[$attr:meta])*
        $name:ident:

        $($body:tt)+
    ) => {
        $(#[$attr])*
        pub fn $name() -> ::std::result::Result<$crate::camino::Utf8PathBuf, $crate::Error> {
            $crate::dir!(_ $($body)+)
        }
    },

    // a shortcut to remove boilerplate when no special code is required to retrieve the base path
    (
        _
        $base:ident

        $($body:tt)+
    ) => {
        $crate::dir!(
            _
            { $base()? }
            $($body)+
        )
    },

    // the additions to the base path
    (
        _
        $base:block
        $(/ #[dir: $is_dir:literal] $new:literal)*
    ) => {
        {
            let mut base = $base;

            $({
                base.push($new);
                $crate::ensure_path(&base, $is_dir)?;
            })*

            ::std::result::Result::Ok(base)
        }
    },
}

dirs_wrapper!(config_local_dir);
dirs_wrapper!(data_local_dir);
dirs_wrapper!(home_dir);

dir! {
    /// TODO
    base_conf_dir:

    config_local_dir
    / #[dir: true] "gdtk"
}

dir! {
    /// TODO
    base_data_dir:

    data_local_dir
    / #[dir: true] "gdtk"
}

dir! {
    /// TODO
    godots_path:

    base_data_dir
    / #[dir: true] "godots"
}

dir! {
    /// TODO
    logs_path:

    base_data_dir
    / #[dir: true] "logs"
}

// we don't use dirs::executable_dir() because it "doesn't work" on Windows
// and macos, even though in practice, `.local/bin` is a thing on all systems

/* TODO: the above is invalid, we should provide a set of env vars to override the
thing in case someone really doesn't want a ~/.local/bin */
dir! {
    /// TODO
    executable_path:

    home_dir
    / #[dir: true] ".local"
    / #[dir: true] "bin"
}

/// Returns the path to the default godot executable.
///
/// **DOES NOT ENSURE THAT IT EXISTS!**
pub fn default_godot_path() -> Result<Utf8PathBuf, Error> {
    let mut exec = executable_path()?;

    // NOTE: while windows won't allow a user to launch any file as an .exe
    // (unless it does end in .exe), it does allow doing so through it's
    // system APIs, so we're good
    exec.push("godot");

    Ok(exec)
}
