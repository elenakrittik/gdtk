use yansi::Paint;

use crate::{
    diagnostic::{Diagnostic, Severity},
    protocol::Visualizer,
    utils::Source,
    Label, Span,
};

const ERROR: yansi::Style = yansi::Style::new().red();
const WARNING: yansi::Style = yansi::Style::new().yellow();
const CUSTOM: yansi::Style = yansi::Style::new().green();
const BORDER: yansi::Style = yansi::Style::new().blue();

#[derive(thiserror::Error, Debug)]
pub enum RustcVisualizerError {
    #[error("IO error: {0}")]
    IOError(#[from] std::io::Error),

    #[error("Line '{0}' (zero-indexed) was not found in the provided source.")]
    LineNotFound(usize),
}

/// A visualizer that visualizes diagnostics in rustc's fashion.
///
/// An example output may look like this:
///
/// ```
/// error[E0499]: cannot borrow `v` as mutable more than once at a time
///  --> src/main.rs:4:15
///   |
/// 3 |     let one = &mut v;
///   |               ------ first mutable borrow occurs here
/// 4 |     let two = &mut v;
///   |               ^^^^^^ second mutable borrow occurs here
/// 5 |
/// 6 |     dbg!(one, two);
///   |          --- first borrow later used here
/// ```
pub struct RustcVisualizer<'a> {
    source_name: &'a str,
    source: Source<'a>,
}

impl<'a> RustcVisualizer<'a> {
    /// Create a new visualizer.
    pub fn new(source_name: &'a str, source: &'a str) -> Self {
        Self {
            source_name,
            source: Source::new(source),
        }
    }
}

impl<'a> Visualizer<'a> for RustcVisualizer<'a> {
    type Error = RustcVisualizerError;

    fn visualize(
        &self,
        diag: Diagnostic<'_>,
        f: &mut impl std::io::Write,
    ) -> Result<(), Self::Error> {
        let Diagnostic {
            message,
            severity,
            code,
            span,
            labels,
            help: _,
        } = diag;

        self.visualize_primary_error(severity, code, message, f)?;
        writeln!(f)?;
        self.visualize_source_pointer(span, f)?;

        for Label { message, span } in labels {
            let Some((line, _)) = self.source.locate(span) else {
                continue;
            };

            s
        }

        writeln!(f)?;

        Ok(())
    }
}

impl<'a> RustcVisualizer<'a> {
    /// Visualize a primary error message.
    ///
    /// Example output may look like this:
    /// ```md
    /// error[E0499]: cannot borrow `v` as mutable more than once at a time
    /// ```
    fn visualize_primary_error(
        &self,
        severity: Severity,
        code: Option<&str>,
        message: &str,
        f: &mut impl std::io::Write,
    ) -> Result<(), <Self as Visualizer<'a>>::Error> {
        let (directive, style) = match severity {
            Severity::Error => ("error", ERROR),
            Severity::Warning => ("warning", WARNING),
            Severity::Custom(kind) => (kind, CUSTOM),
        };

        write!(f, "{}", directive.paint(style))?;

        if let Some(code) = code {
            write!(f, "{}", '['.paint(style))?;
            write!(f, "{}", code.paint(style))?;
            write!(f, "{}", ']'.paint(style))?;
        }

        write!(f, "{}", ": ".paint(style))?;
        write!(f, "{}", message.paint(style))?;

        Ok(())
    }

    /// Visualize a "source pointer".
    ///
    /// Example output may look like this:
    /// ```md
    ///  --> src/main.rs:4:15
    /// ```
    fn visualize_source_pointer(
        &self,
        span: Option<&Span>,
        f: &mut impl std::io::Write,
    ) -> Result<(), <Self as Visualizer<'a>>::Error> {
        write!(f, "{}", "  --> ".paint(BORDER))?;
        write!(f, "{}", self.source_name)?;

        // #![feature(let_chains)], i miss you so much
        if let Some(span) = span {
            if let Some((line, column)) = self.source.locate(&span) {
                write!(f, ":{}:{}", line, column)?;
            }
        }

        Ok(())
    }

    /// Visualize a source line (one-based).
    ///
    /// Example output may look like this:
    /// ```md
    ///  3 |     let one = &mut v;
    /// ```
    fn visualize_source_line(
        &self,
        line: usize,
        f: &mut impl std::io::Write,
    ) -> Result<(), <Self as Visualizer<'a>>::Error> {
        let Some(line_source) = self.source.line(line - 1) else {
            return Err(RustcVisualizerError::LineNotFound(line - 1));
        };

        write!(f, "{}", line.paint(BORDER))?;
        write!(f, " {} ", "|".paint(BORDER))?;
        write!(f, "{}", line_source)?;

        Ok(())
    }
}
