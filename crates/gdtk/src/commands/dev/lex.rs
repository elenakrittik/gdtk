use std::path::PathBuf;

use itertools::Itertools;

use crate::commands::dev::get_content;

pub fn run(file: PathBuf) -> anyhow::Result<()> {
    let content = get_content(file)?;
    let lexed = gdtk_lexer::lex(&content);

    eprintln!("Lexer output:\n```ron\n{:#?}\n```", &lexed.collect_vec());

    Ok(())
}
