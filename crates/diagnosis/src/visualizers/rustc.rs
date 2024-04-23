use yansi::Paint;

use crate::{
    diagnostic::{Diagnostic, Severity},
    protocol::Visualizer,
    utils::Source,
    Span,
};

const ERROR: yansi::Style = yansi::Style::new().red();
const WARNING: yansi::Style = yansi::Style::new().yellow();
const ADVICE: yansi::Style = yansi::Style::new().green();
const BORDER: yansi::Style = yansi::Style::new().blue();

const fn severity_style(severity: &Severity) -> yansi::Style {
    match severity {
        Severity::Error => ERROR,
        Severity::Warning => WARNING,
        Severity::Advice => ADVICE,
    }
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
    type Error = std::io::Error;

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
            labels: _,
            help: _,
        } = diag;

        self.visualize_primary_error(severity, code, message, f)?;

        writeln!(f)?;

        self.visualize_source_pointer(span, f)?;

        writeln!(f)?;

        Ok(())
    }
}

impl<'a> RustcVisualizer<'a> {
    fn visualize_primary_error(
        &self,
        severity: Severity,
        code: Option<&str>,
        message: &str,
        f: &mut impl std::io::Write,
    ) -> Result<(), <Self as Visualizer<'a>>::Error> {
        let style = severity_style(&severity);
        let directive = match severity {
            Severity::Error => "error",
            Severity::Warning => "warning",
            Severity::Advice => "advice",
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

    fn visualize_source_pointer(
        &self,
        span: Option<&Span>,
        f: &mut impl std::io::Write,
    ) -> Result<(), <Self as Visualizer<'a>>::Error> {
        write!(f, "{}", "  --> ".paint(BORDER))?;
        write!(f, "{}", self.source_name)?;

        if let Some(span) = span {
            let (line, column) = self.source.locate(&span);

            write!(f, ":{}:{}", line, column)?;
        }

        Ok(())
    }
}
