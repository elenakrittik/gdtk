use yansi::Paint;

use crate::{
    diagnostic::{Diagnostic, Severity},
    protocol::Visualizer,
    utils::Source,
    Highlight, Span,
};

const ERROR: yansi::Style = yansi::Style::new().red();
const WARNING: yansi::Style = yansi::Style::new().yellow();
const CUSTOM: yansi::Style = yansi::Style::new().green();
const HELP: yansi::Style = yansi::Style::new().blue();
const BORDER: yansi::Style = yansi::Style::new().cyan();

#[derive(thiserror::Error, Debug)]
pub enum RustcVisualizerError {
    #[error("IO error: {0}")]
    IOError(#[from] std::io::Error),

    #[error("Line '{0}' (zero-indexed) was not found in the provided source.")]
    LineNotFound(usize),
}

/// Styles to apply to parts of the output.
pub struct Styles {
    pub error: yansi::Style,
    pub warning: yansi::Style,
    pub custom: yansi::Style,
    pub help: yansi::Style,
    pub border: yansi::Style,
}

impl Default for Styles {
    fn default() -> Self {
        Self {
            error: ERROR,
            warning: WARNING,
            custom: CUSTOM,
            help: HELP,
            border: BORDER,
        }
    }
}

/// A visualizer that visualizes diagnostics in rustc's fashion.
///
/// An example output may look like this:
///
/// ```md
/// warning[invalid-assignment-target]: Invalid assignment target.
///  --> .\quick.gd:2:4
///   |
/// 2 |     2 + 2 = 5
///   |             - ..while trying to assign this expression
/// 2 |     2 + 2 = 5
///   |     ----- ..to this target expression
/// ```
pub struct RustcVisualizer<'a> {
    source_name: &'a str,
    source: Source<'a>,
    styles: Styles,
}

impl<'a> RustcVisualizer<'a> {
    /// Create a new visualizer.
    pub fn new(source_name: &'a str, source: &'a str) -> Self {
        Self {
            source_name,
            source: Source::new(source),
            styles: Styles::default(),
        }
    }

    /// Update colors used by the visualizer.
    pub fn with_styles(mut self, styles: Styles) -> Self {
        self.styles = styles;
        self
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
            highlights,
            help_messages,
        } = diag;

        self.visualize_primary_error(severity, code, message, f)?;
        writeln!(f)?;
        self.visualize_source_pointer(span, f)?;

        if !highlights.is_empty() {
            writeln!(f)?;
            self.visualize_border(None, f)?;
        }

        for Highlight { span, message } in highlights {
            let Some((line, offset)) = self.source.locate(span) else {
                continue;
            };

            writeln!(f)?;
            self.visualize_source_line(line, f)?;
            writeln!(f)?;
            self.visualize_highlight(offset, span.end - span.start, message, f)?;
        }

        if !help_messages.is_empty() {
            writeln!(f)?;
            self.visualize_border(None, f)?;
        }

        for help_message in help_messages {
            writeln!(f)?;
            self.visualize_help_message(help_message, f)?;
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
    /// warning[invalid-assignment-target]: Invalid assignment target.
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
    ///  --> .\quick.gd:2:4
    /// ```
    fn visualize_source_pointer(
        &self,
        span: Option<&Span>,
        f: &mut impl std::io::Write,
    ) -> Result<(), <Self as Visualizer<'a>>::Error> {
        write!(f, " {}", "--> ".paint(BORDER))?;
        write!(f, "{}", self.source_name)?;

        // #![feature(let_chains)], i miss you so much
        if let Some(span) = span {
            if let Some((line, column)) = self.source.locate(span) {
                write!(f, ":{}:{}", line, column)?;
            }
        }

        Ok(())
    }

    /// Visualize a source line (one-based).
    ///
    /// Example output may look like this:
    /// ```md
    /// 2 |     2 + 2 = 5
    /// ```
    fn visualize_source_line(
        &self,
        line: usize,
        f: &mut impl std::io::Write,
    ) -> Result<(), <Self as Visualizer<'a>>::Error> {
        let Some(line_source) = self.source.line(line - 1) else {
            return Err(RustcVisualizerError::LineNotFound(line - 1));
        };

        self.visualize_border(Some(line), f)?;
        write!(f, "{}", line_source)?;

        Ok(())
    }

    /// Visualize a source line (one-based).
    ///
    /// Example output may look like this:
    /// ```md
    ///   |             - ..while trying to assign this expression
    /// ```
    fn visualize_highlight(
        &self,
        offset: usize,
        len: usize,
        message: Option<&str>,
        f: &mut impl std::io::Write,
    ) -> Result<(), <Self as Visualizer<'a>>::Error> {
        self.visualize_border(None, f)?;
        // std::iter::repeat_n but stable
        write!(f, "{}", " ".repeat(offset))?;
        write!(f, "{}", "-".repeat(len).paint(BORDER))?;

        if let Some(message) = message {
            write!(f, " {}", message.paint(BORDER))?;
        }

        Ok(())
    }

    /// Visualize a help message.
    ///
    /// Example output may look like this:
    /// ```md
    ///    = help: assignment chains are not valid syntax
    /// ```
    fn visualize_help_message(
        &self,
        message: &str,
        f: &mut impl std::io::Write,
    ) -> Result<(), <Self as Visualizer<'a>>::Error> {
        write!(f, "{}", "  = help: ".paint(HELP))?;
        write!(f, "{}", message.paint(HELP))?;

        Ok(())
    }

    /// Visualize a help message.
    ///
    /// Example output may look like this:
    /// ```md
    ///  2 |
    /// ```
    fn visualize_border(
        &self,
        num: Option<usize>,
        f: &mut impl std::io::Write,
    ) -> Result<(), <Self as Visualizer<'a>>::Error> {
        if let Some(num) = num {
            write!(f, "{}", num.paint(BORDER))?;
        } else {
            write!(f, " ")?;
        }

        write!(f, " {} ", "|".paint(BORDER))?;

        Ok(())
    }
}
