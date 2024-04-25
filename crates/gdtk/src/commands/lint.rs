use std::path::PathBuf;

use diagnosis::protocol::Visualizer;

use crate::utils::get_content;

pub fn run(file: PathBuf) -> anyhow::Result<()> {
    let content = get_content(file.as_path())?;
    let lexed = gdtk_lexer::lex(&content);
    let parsed = gdtk_parser::parse_file(lexed);

    let source_name = match file.to_str().unwrap() {
        "-" => "<stdin>",
        other => other,
    };

    let diagnostics = gdtk_lint::run_builtin_lints(&parsed);
    let vis = diagnosis::visualizers::rustc::RustcVisualizer::new(source_name, &content);
    let mut stderr = std::io::stderr().lock();

    for diagnostic in diagnostics {
        vis.visualize(diagnostic, &mut stderr)?;
    }

    Ok(())
}
