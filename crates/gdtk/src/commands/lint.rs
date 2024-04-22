use std::path::PathBuf;

use crate::utils::get_content;

pub fn run(file: PathBuf) -> anyhow::Result<()> {
    let content = get_content(file.as_path())?;
    let lexed = gdtk_lexer::lex(&content);
    let parsed = gdtk_parser::parse_file(lexed);

    let diagnostics = gdtk_lint::run_builtin_lints(&parsed);

    for diagnostic in diagnostics {
        let report = miette::Report::new(diagnostic).with_source_code(miette::NamedSource::new(
            file.to_str().unwrap(),
            content.clone(),
        ));

        eprintln!("{:?}", report);
    }

    Ok(())
}
