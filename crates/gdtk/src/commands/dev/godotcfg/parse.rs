use std::path::PathBuf;

use crate::utils::get_content;

pub fn run(file: PathBuf) -> anyhow::Result<()> {
    let content = get_content(file.as_path())?;
    let parsed = gdtk_godotcfg_parser::parser(&content);

    for line in parsed {
        println!("{:?}", line);
    }

    Ok(())
}
