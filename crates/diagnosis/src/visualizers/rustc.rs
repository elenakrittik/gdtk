use count_digits::CountDigits;
use yansi::Paint;

use crate::{
    diagnostic::{Diagnostic, Severity},
    protocol::Visualizer,
    utils::Source,
};

const MAIN_TEXT: yansi::Style = yansi::Style::new().white().bold();
const ERROR: yansi::Style = yansi::Style::new().bright_red().bold();
const WARNING: yansi::Style = yansi::Style::new().yellow().bold();
const CUSTOM: yansi::Style = yansi::Style::new().green().bold();
const HELP: yansi::Style = yansi::Style::new().cyan().bold();
const BORDER: yansi::Style = yansi::Style::new().bright_blue().bold();

#[derive(thiserror::Error, Debug)]
pub enum RustcVisualizerError {
    #[error("IO error: {0}")]
    IOError(#[from] std::io::Error),

    #[error("Line '{0}' (zero-indexed) was not found in the provided source.")]
    LineNotFound(usize),
}

/// Styles to apply to parts of the output.
pub struct Styles {
    pub main_text: yansi::Style,
    pub error: yansi::Style,
    pub warning: yansi::Style,
    pub custom: yansi::Style,
    pub help: yansi::Style,
    pub border: yansi::Style,
}

impl Default for Styles {
    fn default() -> Self {
        Self {
            main_text: MAIN_TEXT,
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

impl<'a, F: std::io::Write> Visualizer<'a, F> for RustcVisualizer<'a> {
    type Error = RustcVisualizerError;

    // TODO: Handle multiple highlights on the same line.
    fn visualize(&self, diag: Diagnostic<'_>, f: &mut F) -> Result<(), Self::Error> {
        // A map of highlight spans to their positions.
        let span_to_pos = ahash::AHashMap::from_iter(
            diag.highlights
                .iter()
                .filter_map(|h| Some(h.span).zip(self.source.locate(h.span))),
        );

        // Offset from the right to account for the line number.
        let line_number_offset = span_to_pos
            .values()
            .map(|(line, _)| line)
            .max()
            .unwrap_or(&0)
            .count_digits();

        let (directive, directive_style) = match diag.severity {
            Severity::Error => ("error", self.styles.error),
            Severity::Warning => ("warning", self.styles.warning),
            Severity::Custom(directive) => (directive, self.styles.custom),
        };

        // Step 1. Draw the primary messsage.
        write!(f, "{}", directive.paint(directive_style))?;

        if let Some(code) = diag.code {
            write!(f, "{}", '['.paint(directive_style))?;

            write!(f, "{}", code.paint(directive_style))?;
            write!(f, "{}", ']'.paint(directive_style))?;
        }

        write!(f, "{}", ":".paint(self.styles.main_text))?;
        write!(f, " ")?;
        write!(f, "{}", diag.message.paint(self.styles.main_text))?;
        writeln!(f)?;

        // Step 2. Draw the source pointer.
        write!(f, "{}", " ".repeat(line_number_offset))?;
        write!(f, "{}", "-->".paint(self.styles.border))?;
        write!(f, " ")?;
        write!(f, "{}", self.source_name)?;

        if let Some(span) = diag.span
            && let Some((line, column)) = self.source.locate(span)
        {
            write!(f, ":{}:{}", line, column)?;
        }

        // Step 3. Draw highlights.

        for highlight in diag.highlights {
            let Some((line, column)) = span_to_pos.get(&highlight.span) else {
                continue;
            };

            let Some(line_source) = self.source.line(*line) else {
                continue;
            };

            let local_line_number_offset = line_number_offset + 1 - line.count_digits();

            writeln!(f)?;

            // Step 3.1. Draw an empty "separator" border.
            write!(f, "{}", " ".repeat(line_number_offset + 1))?;
            write!(f, "{}", "|".paint(self.styles.border))?;
            writeln!(f)?;

            // Step 3.2. Draw the highlighted source line.
            write!(f, "{}", line.paint(self.styles.border))?;
            write!(f, "{}", " ".repeat(local_line_number_offset))?;
            write!(f, "{}", "|".paint(self.styles.border))?;
            write!(f, " ")?;
            write!(f, "{}", line_source)?;
            writeln!(f)?;

            // Step 3.3. Draw the highlight.
            write!(f, "{}", " ".repeat(line_number_offset + 1))?;
            write!(f, "{}", "|".paint(self.styles.border))?;

            // God did not intend for tabs to exist (joke (or is it?))
            // Tabs are displayed differently in every terminal, thus
            // simply using `" ".repeat(column + 1)` won't work as a tab
            // is rarely of the same width as a space. To fix this, we
            // need to emit `(column - n_tabs)` spaces and `(n_tabs)`
            // tabs (under the assumption that all other characters are
            // "1-wide"), so that the total visible width is the same as
            // in the line source.
            let n_tabs = line_source[..*column]
                .chars()
                .filter(|c| c == &'\t')
                .count();

            write!(f, "{}", " ".repeat(column - n_tabs))?;
            write!(f, "{}", "\t".repeat(n_tabs))?;

            let span_length = highlight.span.end - highlight.span.start;
            write!(f, "{}", "-".repeat(span_length).paint(directive_style))?;

            if let Some(message) = highlight.message {
                write!(f, " ")?;
                write!(f, "{}", message.paint(directive_style))?;
            }
        }

        // Step 4. Draw help messages.
        for help_message in diag.help_messages {
            writeln!(f)?;

            write!(f, "{}", " ".repeat(line_number_offset + 1))?;
            write!(f, "{}", "=".paint(self.styles.help))?;
            write!(f, " ")?;
            write!(f, "{}", "help:".paint(self.styles.help))?;
            write!(f, " ")?;
            write!(f, "{}", help_message.paint(self.styles.help))?;
        }

        Ok(())
    }
}
