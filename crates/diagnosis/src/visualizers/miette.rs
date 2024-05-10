pub use miette;

use crate::{Diagnostic, Highlight, Severity, Visualizer};

#[derive(Debug, thiserror::Error)]
pub enum MietteVisualizerError {
    #[error("fmt error: {0}")]
    Fmt(#[from] std::fmt::Error),

    #[error("IO error: {0}")]
    IO(#[from] std::io::Error),
}

pub struct MietteVisualizer<'a> {
    handler: miette::GraphicalReportHandler,
    source_name: &'a str,
    source: &'a str,
}

impl<'a> MietteVisualizer<'a> {
    pub fn new(source_name: &'a str, source: &'a str) -> Self {
        Self {
            handler: miette::GraphicalReportHandler::default(),
            source_name,
            source,
        }
    }

    pub fn with_theme(mut self, theme: miette::GraphicalTheme) -> Self {
        self.handler = self.handler.with_theme(theme);
        self
    }
}

impl<'a> Visualizer<'a> for MietteVisualizer<'a> {
    type Error = MietteVisualizerError;

    fn visualize(
        &self,
        diag: Diagnostic<'_>,
        f: &mut impl std::io::Write,
    ) -> Result<(), Self::Error> {
        let better = BetterMietteDiagnostic::from_diag(diag, self.source, self.source_name);

        let mut buf = String::new();

        self.handler.render_report(&mut buf, &better)?;

        write!(f, "{}", buf.trim_end())?;

        Ok(())
    }
}

#[derive(Debug, thiserror::Error)]
#[error("idk")]
struct BetterMietteDiagnostic<'a> {
    message: &'a str,
    code: Option<&'a str>,
    severity: Severity<'a>,
    help_messages: Vec<&'a str>,
    highlights: Vec<Highlight<'a>>,
    source_name: &'a str,
    source_: &'a str,
}

impl<'a> BetterMietteDiagnostic<'a> {
    fn from_diag(diag: Diagnostic<'a>, source_: &'a str, source_name: &'a str) -> Self {
        Self {
            message: diag.message,
            code: diag.code,
            severity: diag.severity,
            help_messages: diag.help_messages,
            highlights: diag.highlights,
            source_name,
            source_,
        }
    }
}

impl miette::Diagnostic for BetterMietteDiagnostic<'_> {
    fn code<'a>(&'a self) -> Option<Box<dyn std::fmt::Display + 'a>> {
        Some(Box::new(self.code?))
    }

    fn severity(&self) -> Option<miette::Severity> {
        Some(match self.severity {
            Severity::Error => miette::Severity::Error,
            Severity::Warning => miette::Severity::Warning,
            Severity::Custom(_) => miette::Severity::Advice,
        })
    }

    fn help<'a>(&'a self) -> Option<Box<dyn std::fmt::Display + 'a>> {
        let help = self.help_messages.join("\n");

        if help.is_empty() {
            None
        } else {
            Some(Box::new(help))
        }
    }

    fn url<'a>(&'a self) -> Option<Box<dyn std::fmt::Display + 'a>> {
        None
    }

    fn source_code(&self) -> Option<&dyn miette::SourceCode> {
        Some(&self.source_)
    }

    fn labels(&self) -> Option<Box<dyn Iterator<Item = miette::LabeledSpan> + '_>> {
        if self.highlights.is_empty() {
            None
        } else {
            Some(Box::new(self.highlights.iter().map(|h| {
                miette::LabeledSpan::new(
                    h.message.map(ToString::to_string),
                    h.span.start,
                    h.span.end - h.span.start,
                )
            })))
        }
    }

    fn related<'a>(&'a self) -> Option<Box<dyn Iterator<Item = &'a dyn miette::Diagnostic> + 'a>> {
        None
    }

    fn diagnostic_source(&self) -> Option<&dyn miette::Diagnostic> {
        None
    }
}
