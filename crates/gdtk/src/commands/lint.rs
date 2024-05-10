use std::{
    io::Write,
    path::{Path, PathBuf},
};

use diagnosis::protocol::Visualizer;

use crate::utils::{get_content, resolve_files_by_ext};

pub fn run(files: Vec<PathBuf>) -> anyhow::Result<()> {
    let files = resolve_files_by_ext(files, "gd")?;
    let counts: Counter = files.iter().filter_map(|p| run_on_file(p).ok()).sum();

    if counts.errors > 0 || counts.warnings > 0 {
        eprintln!(
            "Checked {} files, {} errors, {} warnings.",
            files.len(),
            counts.errors,
            counts.warnings,
        );
    } else {
        eprintln!(
            "Checked {} files, nothing found. Enjoy your day! ✨️",
            files.len(),
        );
    }

    Ok(())
}

fn run_on_file(file: &Path) -> anyhow::Result<Counter> {
    let content = get_content(file)?;
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
    let mut counter = Counter::default();
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

        match diagnostic.severity {
            diagnosis::Severity::Error => counter.errors += 1,
            diagnosis::Severity::Warning => counter.warnings += 1,
            _ => (),
        }

        vis.visualize(diagnostic, &mut stderr)?;
        write!(stderr, "\n\n")?;
    }

    Ok(counter)
}

#[derive(Default)]
struct Counter {
    errors: usize,
    warnings: usize,
}

impl std::ops::Add for Counter {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            errors: self.errors + rhs.errors,
            warnings: self.warnings + rhs.warnings,
        }
    }
}

impl std::iter::Sum for Counter {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Counter::default(), std::ops::Add::add)
    }
}
