use std::{io::{Error, ErrorKind}, path::PathBuf};

pub fn ensure_path(path: &PathBuf, dir: bool) -> Result<bool, Error> {
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
