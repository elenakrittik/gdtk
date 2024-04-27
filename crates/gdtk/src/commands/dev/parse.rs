use std::path::PathBuf;

use crate::utils::get_content;

pub fn run(file: PathBuf) -> anyhow::Result<()> {
    let content = get_content(file.as_path())?;
    let lexed = gdtk_parser::lexer::lex(&content);
    let parsed = gdtk_parser::parse_file(lexed);

    eprintln!("Parser output:\n```ron\n{:#?}\n```", &parsed);

    Ok(())
}
