use std::path::PathBuf;

pub fn run(file: PathBuf) -> anyhow::Result<()> {
    let content = std::fs::read_to_string(file)?;

    eprintln!("Source:\n```gdscript\n{}\n```", &content);

    let lexed = gdtk_lexer::lex(&content);

    eprintln!("Lexer output:\n```ron\n{:#?}\n```", &lexed.0);

    let parsed = gdtk_parser::parse_file(lexed)?;

    eprintln!("Parser output:\n```ron\n{:#?}\n```", &parsed);

    Ok(())
}
