use std::{
    io::{Error, ErrorKind},
    path::{Path, PathBuf},
};

pub fn ensure_path(path: &Path, dir: bool) -> Result<bool, Error> {
    if path.exists() {
        return Ok(false);
    }

    if dir {
        std::fs::create_dir(path)?;
    } else {
        std::fs::OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(path)?;
    }

    Ok(true)
}

pub fn base_conf_dir() -> Result<PathBuf, Error> {
    let mut conf_dir = dirs::config_local_dir().ok_or(Error::new(
        ErrorKind::NotFound,
        "Config directory could not be detected.",
    ))?;

    conf_dir.push("gdtk");
    ensure_path(&conf_dir, true)?;

    Ok(conf_dir)
}

pub fn base_data_dir() -> Result<PathBuf, Error> {
    let mut data_dir = dirs::data_local_dir().ok_or(Error::new(
        ErrorKind::NotFound,
        "Data directory could not be detected.",
    ))?;

    data_dir.push("gdtk");
    ensure_path(&data_dir, true)?;

    Ok(data_dir)
}

pub fn versions_toml_path() -> Result<PathBuf, std::io::Error> {
    let mut conf_dir = base_conf_dir()?;

    conf_dir.push("versions.toml");

    if ensure_path(&conf_dir, false)? {
        std::fs::write(&conf_dir, "[versions]")?;
    }

    Ok(conf_dir)
}

pub fn godots_path() -> Result<PathBuf, std::io::Error> {
    let mut data_dir = base_data_dir()?;

    data_dir.push("godots");

    ensure_path(&data_dir, true)?;

    Ok(data_dir)
}

pub fn logs_path() -> Result<PathBuf, std::io::Error> {
    let mut data_dir = base_data_dir()?;

    data_dir.push("logs");

    ensure_path(&data_dir, true)?;

    Ok(data_dir)
}

pub fn executable_path() -> Result<PathBuf, std::io::Error> {
    // we don't use dirs::executable_dir() because it "doesn't work" on Windows
    // and macos, even though in practice, `.local/bin` is a thing on all systems

    let mut home = dirs::home_dir().ok_or(std::io::Error::new(
        std::io::ErrorKind::NotFound,
        "Home directory could not be detected.",
    ))?;

    home.push(".local");

    ensure_path(&home, true)?;

    home.push("bin");

    ensure_path(&home, true)?;

    Ok(home)
}

/// Returns the path to the default godot executable.
///
/// **DOES NOT ENSURE THAT IT EXISTS!**
pub fn default_godot_path() -> Result<PathBuf, std::io::Error> {
    let mut exec = executable_path()?;

    // NOTE: windows is dumb and can only know if something is executable by
    // looking at it's extension, so running `godot` *manually* will not work
    // on windows. however, executing it through a symlink or by spawning a
    // process will work just fine, so we are fine as well.
    exec.push("godot");

    Ok(exec)
}
