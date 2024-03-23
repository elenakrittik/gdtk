use std::path::PathBuf;

pub fn run(file: PathBuf) -> anyhow::Result<()> {
    let content = std::fs::read_to_string(file)?;
    let lexed = gdtk_lexer::lex(&content);
    let parsed = gdtk_parser::parse_file(lexed)?;

    eprintln!("Parser output:\n```ron\n{:#?}\n```", &parsed);

    Ok(())
}
