use ahash::AHashMap;
use count_digits::CountDigits;
use itertools::Itertools;
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
        children: Vec<Annotation<'a>>,
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

    fn render(&mut self) -> Result<'_> {
        self.render_primary_message()?;
        self.render_source_pointer()?;
        self.render_highlights()?;
        self.render_help_messages()?;

        Ok(())
    }

    fn render_primary_message(&mut self) -> Result<'_> {
        let (directive, directive_style) = direct(self.styles, &self.diag.severity);

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

    fn render_source_pointer(&mut self) -> Result<'_> {
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

    fn render_highlights(&mut self) -> Result<'_> {
        let annotations = self.annotations();

        for ann in annotations {
            let self_mut = force_mut(self);

            match ann {
                Annotation::Standalone(highlight) => {
                    self_mut.render_standalone_highlight(highlight)?
                }
                Annotation::Singleline(_) => (),
                Annotation::Multiline {
                    highlight: _,
                    children: _,
                } => (),
            }
        }

        Ok(())
    }

    fn render_standalone_highlight(&mut self, highlight: &Highlight<'a>) -> Result {
        let (line, column) = self.source.locate(highlight.span).unwrap();

        let line_source = self.source.line(line).unwrap();

        let local_line_number_offset = self.line_number_offset + 1 - line.count_digits();

        // Step 3.1. Draw an empty "separator" border.
        write!(self.fmt, "{}", " ".repeat(self.line_number_offset + 1))?;
        write!(self.fmt, "{}", "|".paint(self.styles.border))?;
        writeln!(self.fmt)?;

        // Step 3.2. Draw the highlighted source line.
        write!(self.fmt, "{}", line.paint(self.styles.border))?;
        write!(self.fmt, "{}", " ".repeat(local_line_number_offset))?;
        write!(self.fmt, "{}", "|".paint(self.styles.border))?;
        write!(self.fmt, " ")?;
        write!(self.fmt, "{}", line_source.replace('\t', "    "))?;
        writeln!(self.fmt)?;

        // Step 3.3. Draw the highlight.
        write!(self.fmt, "{}", " ".repeat(self.line_number_offset + 1))?;
        write!(self.fmt, "{}", "|".paint(self.styles.border))?;

        // God did not intend for tabs to exist (joke (or is it?))
        // Tabs are displayed differently in every terminal, thus
        // simply using `" ".repeat(column + 1)` won't work as a tab
        // is rarely of the same width as a space. To fix this, we
        // need to emit `(column - n_tabs)` spaces and `(n_tabs)`
        // tabs (under the assumption that all other characters are
        // "1-wide"), so that the total visible width is the same as
        // in the line source.
        let n_tabs = line_source[..column].chars().filter(|c| c == &'\t').count();

        write!(self.fmt, "{}", " ".repeat(column - n_tabs))?;
        write!(self.fmt, "{}", "    ".repeat(n_tabs))?;

        let directive_style = direct(self.styles, &self.diag.severity).1;

        let span_length = highlight.span.end - highlight.span.start;
        write!(
            self.fmt,
            "{}",
            "-".repeat(span_length).paint(directive_style)
        )?;

        if let Some(message) = highlight.message {
            write!(self.fmt, " ")?;
            write!(self.fmt, "{}", message.paint(directive_style))?;
        }

        Ok(())
    }

    fn annotations<'b>(&'b self) -> Vec<Annotation<'b>> {
        // First, divide all highlights into multiline and non-multiline. To
        // do that, we first identify multiline highlights, and then sort the
        // remaining ones either into a "standalone" category or into one of
        // the multiline groups.

        let mut non_multilines = ahash::AHashSet::<&Highlight<'a>>::new();
        let mut multilines = self.find_multilines();

        for a_highlight in self.diag.highlights.iter() {
            if multilines.contains_key(a_highlight) {
                continue;
            }

            for (m_highlight, m_values) in multilines.iter_mut() {
                if m_highlight.span.start <= a_highlight.span.start
                    && m_highlight.span.end >= a_highlight.span.end
                {
                    m_values.push(a_highlight);
                    break;
                } else {
                    non_multilines.insert(a_highlight);
                }
            }
        }

        // Then for each vec<highlight> we combine highlights
        // on the same lines

        let mut annotations = vec![];

        annotations.extend(self.group_by_line(non_multilines.into_iter()));

        for (highlight, children) in multilines {
            let singleline = self.group_by_line(children.into_iter());

            annotations.push(Annotation::Multiline {
                highlight,
                children: singleline,
            });
        }

        annotations
    }

    fn group_by_line<'b>(
        &'b self,
        highlights: impl Iterator<Item = &'b Highlight<'b>>,
    ) -> Vec<Annotation<'b>> {
        highlights
            .chunk_by(|h| self.source.locate(h.span).map(|(line, _)| line))
            .into_iter()
            .map(|(_, group)| {
                let on_one_line: Vec<_> = group.collect();

                if let [highlight] = on_one_line.as_slice() {
                    Annotation::Standalone(highlight)
                } else {
                    Annotation::Singleline(on_one_line)
                }
            })
            .collect()
    }

    fn find_multilines<'b>(&'b self) -> ahash::AHashMap<&'b Highlight<'a>, Vec<&'b Highlight<'b>>> {
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
                    return left != right;
                }

                false
            })
            .map(|h| (h, Vec::<&'a Highlight<'a>>::new()));

        AHashMap::from_iter(multilines_iter)
    }

    fn render_help_messages(&mut self) -> Result<'_> {
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

fn direct<'a>(styles: &'a Styles, severity: &'a Severity) -> (&'a str, yansi::Style) {
    match severity {
        Severity::Error => ("error", styles.error),
        Severity::Warning => ("warning", styles.warning),
        Severity::Custom(directive) => (directive, styles.custom),
    }
}

fn force_mut<'src_a, 'src_ref, 'out_a: 'src_a, 'out_ref: 'src_ref, F: std::io::Write>(
    renderer: &'src_ref RustcDiagnosticRenderer<'src_a, F>,
) -> &'out_ref mut RustcDiagnosticRenderer<'out_a, F> {
    unsafe {
        (renderer as *const RustcDiagnosticRenderer<'src_a, F> as usize
            as *mut RustcDiagnosticRenderer<'out_a, F>)
            .as_mut()
            .unwrap()
    }
}
