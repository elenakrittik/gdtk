use std::{io::Write, path::PathBuf};

use diagnosis::protocol::Visualizer;

use crate::utils::{get_content, resolve_files_by_ext};

pub fn run(files: Vec<PathBuf>) -> anyhow::Result<()> {
    let files = resolve_files_by_ext(files, "gd")?;

    for file in files {
        run_on_file(file)?;
    }

    Ok(())
}

fn run_on_file(file: PathBuf) -> anyhow::Result<()> {
    let content = get_content(file.as_path())?;
    let noqas = gdtk_lexer::noqas(&content);
    let lexed = gdtk_lexer::lex(&content);
    let parsed = gdtk_parser::parse_file(lexed);

    let source = diagnosis::utils::Source::new(&content);
    let source_name = match file.to_str().unwrap() {
        "-" => "<stdin>",
        other => other,
    };

    let diagnostics = gdtk_lint::run_builtin_lints(&parsed);
    let vis = diagnosis::visualizers::rustc::RustcVisualizer::new(source_name, &content);
    let mut stderr = std::io::stderr().lock();

    for diagnostic in diagnostics {
        if let Some(code) = diagnostic.code
            && let Some(span) = diagnostic.span
            && let Some((line, _)) = source.locate(span)
            && let Some(noqas) = noqas.get(&line)
            && noqas.contains(&code)
        {
            continue;
        }

        vis.visualize(diagnostic, &mut stderr)?;
        write!(stderr, "\n\n")?;
    }

    Ok(())
}
