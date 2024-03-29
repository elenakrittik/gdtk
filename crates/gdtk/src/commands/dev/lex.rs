use std::path::PathBuf;

pub fn run(file: PathBuf) -> anyhow::Result<()> {
    let content = std::fs::read_to_string(file)?;
    let lexed = gdtk_lexer::lex(&content);

    eprintln!("Lexer output:\n```ron\n{:#?}\n```", &lexed.0);

    Ok(())
}
