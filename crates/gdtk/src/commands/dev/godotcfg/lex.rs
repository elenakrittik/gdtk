use std::path::PathBuf;

use crate::utils::get_content;

pub fn run(file: PathBuf) -> anyhow::Result<()> {
    let content = get_content(file.as_path())?;
    let parsed = gdtk_godotcfg_parser::tokens(&content);

    for token in parsed {
        println!("{:?}", token);
    }

    Ok(())
}
