use std::path::PathBuf;

use itertools::Itertools;

use crate::utils::get_content;

pub fn run(file: PathBuf) -> anyhow::Result<()> {
    let content = get_content(file.as_path())?;
    let lexed = gdtk_parser::lexer::lex(&content);

    eprintln!("Lexer output:\n```ron\n{:#?}\n```", &lexed.collect_vec());

    Ok(())
}
