use std::path::PathBuf;

use crate::utils::get_content;

pub fn run(file: PathBuf) -> anyhow::Result<()> {
    let content = get_content(file.as_path())?;
    let lexed = gdtk_godotcfg_parser::lexer(&content);

    for token in lexed {
        println!("{:?}", token);
    }

    Ok(())
}
