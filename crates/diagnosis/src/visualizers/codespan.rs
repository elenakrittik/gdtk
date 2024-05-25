pub use codespan_reporting;

use crate::Visualizer;

pub struct CodespanVisualizer<'a> {
    files: codespan_reporting::files::SimpleFile<&'a str, &'a str>,
}

impl<'a> CodespanVisualizer<'a> {
    /// Create a new visualizer.
    pub fn new(name: &'a str, source: &'a str) -> Self {
        Self {
            files: codespan_reporting::files::SimpleFile::new(name, source),
        }
    }
}

impl<'a, F: std::io::Write + codespan_reporting::term::termcolor::WriteColor> Visualizer<'a, F>
    for CodespanVisualizer<'a>
{
    type Error = codespan_reporting::files::Error;

    fn visualize(&self, diag: crate::Diagnostic<'_>, f: &mut F) -> Result<(), Self::Error> {
        let codespan_diag = codespan_reporting::diagnostic::Diagnostic {
            severity: match diag.severity {
                crate::Severity::Error => codespan_reporting::diagnostic::Severity::Error,
                crate::Severity::Warning => codespan_reporting::diagnostic::Severity::Warning,
                crate::Severity::Custom(_) => codespan_reporting::diagnostic::Severity::Note,
            },
            code: diag.code.map(ToString::to_string),
            message: diag.message.to_string(),
            labels: diag
                .highlights
                .into_iter()
                .map(|highlight| {
                    let mut label = codespan_reporting::diagnostic::Label::secondary(
                        (),
                        highlight.span.clone(),
                    );

                    if let Some(message) = highlight.message {
                        label.message = message.to_string();
                    }

                    label
                })
                .collect(),
            notes: diag
                .help_messages
                .into_iter()
                .map(ToString::to_string)
                .collect(),
        };

        let config = codespan_reporting::term::Config::default();

        codespan_reporting::term::emit(f, &config, &self.files, &codespan_diag).unwrap();

        Ok(())
    }
}
