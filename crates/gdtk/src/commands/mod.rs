pub mod dev;
pub mod godot;
pub mod lint;

use std::{path::PathBuf, io::Read};

pub fn get_content(file: PathBuf) -> anyhow::Result<String> {
    Ok(if file.to_str().is_some_and(|p| p == "-") {
        let mut buf = String::new();
        std::io::stdin().lock().read_to_string(&mut buf)?;

        buf
    } else {
        std::fs::read_to_string(file)?
    })
}
