use std::io::{Error, ErrorKind};

pub fn ensure_path(path: std::path::PathBuf, dir: bool) -> Result<bool, Error> {
    let mut existed = path.exists();

    if !existed {
        if dir {
            std::fs::create_dir(path)?;
        } else {
            std::fs::OpenOptions::new().create(true).write(true).open(path)?;
            existed = false;
        }
    }

    Ok(existed)
}

pub fn base_conf_dir() -> Result<std::path::PathBuf, Error> {
    let mut conf_dir = dirs::config_local_dir().ok_or(Error::new(ErrorKind::NotFound, "Config dirrectory not gound"))?;

    conf_dir.push("gdtk");

    Ok(conf_dir)
}

pub fn base_data_dir() -> Result<std::path::PathBuf, Error> {
    let mut data_dir = dirs::data_local_dir().ok_or(Error::new(ErrorKind::NotFound, "Data dirrectory not gound"))?;

    data_dir.push("gdtk");

    Ok(data_dir)
}
