use std::{io::Read, path::PathBuf};

pub mod lex;
pub mod parse;

fn get_content(file: PathBuf) -> anyhow::Result<String> {
    Ok(if file.to_str().is_some_and(|p| p == "-") {
        let mut buf = String::new();
        std::io::stdin().lock().read_to_string(&mut buf)?;

        buf
    } else {
        std::fs::read_to_string(file)?
    })
}
