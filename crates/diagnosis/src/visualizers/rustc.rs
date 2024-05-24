use ahash::AHashMap;
use count_digits::CountDigits;
use yansi::Paint;

use crate::{
    diagnostic::{Diagnostic, Severity},
    protocol::Visualizer,
    utils::Source,
    Highlight,
};

const MAIN_TEXT: yansi::Style = yansi::Style::new().white().bold();
const ERROR: yansi::Style = yansi::Style::new().bright_red().bold();
const WARNING: yansi::Style = yansi::Style::new().yellow().bold();
const CUSTOM: yansi::Style = yansi::Style::new().green().bold();
const HELP: yansi::Style = yansi::Style::new().cyan().bold();
const BORDER: yansi::Style = yansi::Style::new().bright_blue().bold();

type Result<'a, T = ()> = std::result::Result<T, <RustcVisualizer<'a> as Visualizer<'a>>::Error>;

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

impl<'a> Visualizer<'a> for RustcVisualizer<'a> {
    type Error = RustcVisualizerError;

    fn visualize(&self, diag: Diagnostic<'_>, fmt: &mut impl std::io::Write) -> Result {
        RustcDiagnosticRenderer::new(self, diag, fmt).render()?;

        Ok(())
    }
}

struct RustcDiagnosticRenderer<'a, F: std::io::Write> {
    source_name: &'a str,
    source: &'a Source<'a>,
    styles: &'a Styles,
    diag: Diagnostic<'a>,
    fmt: &'a mut F,
    line_number_offset: usize,
}

#[derive(Debug)]
enum Annotation<'a> {
    Standalone(&'a Highlight<'a>),
    Singleline(Vec<&'a Highlight<'a>>),
    Multiline {
        highlight: &'a Highlight<'a>,
        children: Vec<&'a Highlight<'a>>,
    },
}

impl<'a, F: std::io::Write> RustcDiagnosticRenderer<'a, F> {
    fn new(visualizer: &'a RustcVisualizer<'a>, diag: Diagnostic<'a>, fmt: &'a mut F) -> Self {
        // Offset from the right to account for the line number.
        let line_number_offset = diag
            .highlights
            .iter()
            .filter_map(|h| visualizer.source.locate(h.span))
            .map(|(line, _)| line)
            .max()
            .unwrap_or(0)
            .count_digits();

        Self {
            source_name: visualizer.source_name,
            source: &visualizer.source,
            styles: &visualizer.styles,
            diag,
            fmt,
            line_number_offset,
        }
    }

    fn render(&mut self) -> Result {
        self.render_primary_message()?;
        self.render_source_pointer()?;
        self.render_highlights()?;
        self.render_help_messages()?;

        Ok(())
    }

    fn render_primary_message(&mut self) -> Result {
        let (directive, directive_style) = match self.diag.severity {
            Severity::Error => ("error", self.styles.error),
            Severity::Warning => ("warning", self.styles.warning),
            Severity::Custom(directive) => (directive, self.styles.custom),
        };

        write!(self.fmt, "{}", directive.paint(directive_style))?;

        if let Some(code) = self.diag.code {
            write!(self.fmt, "{}", '['.paint(directive_style))?;
            write!(self.fmt, "{}", code.paint(directive_style))?;
            write!(self.fmt, "{}", ']'.paint(directive_style))?;
        }

        write!(self.fmt, "{}", ":".paint(self.styles.main_text))?;
        write!(self.fmt, " ")?;
        write!(
            self.fmt,
            "{}",
            self.diag.message.paint(self.styles.main_text)
        )?;
        writeln!(self.fmt)?;

        Ok(())
    }

    fn render_source_pointer(&mut self) -> Result {
        write!(self.fmt, "{}", " ".repeat(self.line_number_offset))?;
        write!(self.fmt, "{}", "-->".paint(self.styles.border))?;
        write!(self.fmt, " ")?;
        write!(self.fmt, "{}", self.source_name)?;

        if let Some(span) = self.diag.span
            && let Some((line, column)) = self.source.locate(span)
        {
            write!(self.fmt, ":{}:{}", line, column)?;
        }

        Ok(())
    }

    fn render_highlights(&mut self) -> Result {
        let annotations = self.annotations();

        dbg!(&annotations);

        Ok(())
    }

    fn annotations(&'a self) -> Vec<Annotation<'a>> {
        self.multiline_groups()
    }

    fn multiline_groups(&'a self) -> Vec<Annotation<'a>> {
        // First, divide all highlight into multiline groups. To do that, we
        // first identify highlights that are multiline, and then sort the
        // remaining ones either into a "standalone" category or into one of
        // the multiline groups.
        let mut standalones: Vec<&Highlight<'a>> = vec![];
        let mut multilines = self.find_multilines();

        for a_highlight in self.diag.highlights.iter() {
            if multilines.contains_key(a_highlight) {
                continue;
            }

            for (m_highlight, m_values) in multilines.iter_mut() {
                if m_highlight.span.start >= a_highlight.span.start
                    && m_highlight.span.end <= a_highlight.span.end
                {
                    m_values.push(a_highlight);
                } else {
                    standalones.push(a_highlight);
                }
            }
        }

        let mut annotations = vec![];

        for (highlight, children) in multilines {
            annotations.push(Annotation::Multiline {
                highlight,
                children,
            });
        }

        for standalone in standalones {
            annotations.push(Annotation::Standalone(standalone));
        }

        annotations
    }

    fn find_multilines(&'a self) -> ahash::AHashMap<&'a Highlight<'a>, Vec<&'a Highlight<'a>>> {
        let multilines_iter = self
            .diag
            .highlights
            .iter()
            .filter(|h| {
                let left = h.span.start..=h.span.start;
                let right = h.span.end..=h.span.end;

                if let Some((left, _)) = self.source.locate(&left)
                    && let Some((right, _)) = self.source.locate(&right)
                {
                    return left == right;
                }

                false
            })
            .map(|h| (h, Vec::<&'a Highlight<'a>>::new()));

        dbg!(AHashMap::from_iter(multilines_iter))
    }

    fn render_help_messages(&mut self) -> Result {
        for help_message in &self.diag.help_messages {
            writeln!(self.fmt)?;

            write!(self.fmt, "{}", " ".repeat(self.line_number_offset + 1))?;
            write!(self.fmt, "{}", "=".paint(self.styles.help))?;
            write!(self.fmt, " ")?;
            write!(self.fmt, "{}", "help:".paint(self.styles.help))?;
            write!(self.fmt, " ")?;
            write!(self.fmt, "{}", help_message.paint(self.styles.help))?;
        }

        Ok(())
    }
}
