use std::io::{Error, ErrorKind};

pub fn ensure_path(path: &std::path::PathBuf, dir: bool) -> Result<(), Error> {
    if path.exists() {
        return Ok(());
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

    Ok(())
}

pub fn base_conf_dir() -> Result<std::path::PathBuf, Error> {
    let mut conf_dir = dirs::config_local_dir().ok_or(Error::new(
        ErrorKind::NotFound,
        "Config directory could not be detected.",
    ))?;

    conf_dir.push("gdtk");
    ensure_path(&conf_dir, true)?;

    Ok(conf_dir)
}

pub fn base_data_dir() -> Result<std::path::PathBuf, Error> {
    let mut data_dir = dirs::data_local_dir().ok_or(Error::new(
        ErrorKind::NotFound,
        "Data directory could not be detected.",
    ))?;

    data_dir.push("gdtk");
    ensure_path(&data_dir, true)?;

    Ok(data_dir)
}
