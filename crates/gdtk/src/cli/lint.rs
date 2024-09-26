use std::{
    io::Write,
    path::{Path, PathBuf},
};

use diagnosis::protocol::Visualizer;

use super::unknown;
use crate::utils::{get_content, resolve_files_by_ext};

pub struct LintCommand {
    pub files: Vec<PathBuf>,
}

impl tapcli::Command for LintCommand {
    type Error = anyhow::Error;

    fn parse(parser: &mut tapcli::Parser) -> Result<Self, Self::Error> {
        let mut files = Vec::new();

        for arg in parser {
            match arg {
                tapcli::Arg::Value(path) => files.push(path.into()),
                other => unknown!(other),
            }
        }

        Ok(Self { files })
    }

    fn run(self) -> Result<Self::Output, Self::Error> {
        let files = resolve_files_by_ext(self.files, "gd")?;
        let counts: Counter = files.iter().filter_map(|p| run_on_file(p).ok()).sum();

        if counts.errors > 0 || counts.warnings > 0 {
            eprintln!(
                "Checked {} file(s), {} errors, {} warnings.",
                files.len(),
                counts.errors,
                counts.warnings,
            );
        } else {
            eprintln!(
                "Checked {} file(s), nothing found. Enjoy your day! ✨️",
                files.len(),
            );
        }

        Ok(())
    }
}

fn run_on_file(file: &Path) -> anyhow::Result<Counter> {
    let content = get_content(file)?;
    let noqas = gdtk_gdscript_parser::lexer::noqas(&content);
    let lexed = gdtk_gdscript_parser::lexer::lex(&content);
    let parsed = gdtk_gdscript_parser::parse_file(lexed);

    let source = diagnosis::utils::Source::new(&content);
    let source_name = match file.to_str().unwrap() {
        "-" => "<stdin>",
        other => other,
    };

    let diagnostics = gdtk_lint::run_builtin_lints(&parsed);
    let vis = diagnosis::visualizers::codespan::CodespanVisualizer::new(source_name, &content);
    let mut counter = Counter::default();
    let mut stderr = diagnosis::visualizers::codespan::codespan_reporting::term::termcolor::StandardStream::stderr(diagnosis::visualizers::codespan::codespan_reporting::term::termcolor::ColorChoice::Always);

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
