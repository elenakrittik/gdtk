use std::path::PathBuf;

use crate::commands::get_content;

pub fn run(file: PathBuf) -> anyhow::Result<()> {
    let content = get_content(file)?;
    let lexed = gdtk_lexer::lex(&content);
    let parsed = gdtk_parser::parse_file(lexed)?;

    eprintln!("Parser output:\n```ron\n{:#?}\n```", &parsed);

    Ok(())
}
