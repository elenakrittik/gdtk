use std::{path::PathBuf, rc::Rc};

use itertools::Itertools;

use crate::utils::get_content;

pub fn run(file: PathBuf) -> anyhow::Result<()> {
    let content = get_content(file.as_path())?;
    let lexed = gdtk_lexer::lex(&content);
    let parsed = gdtk_parser::parse_file(lexed);

    let diagnostics = gdtk_lint::run_builtin_lints(&parsed);

    for diagnostic in diagnostics {
        let report = miette::Report::new(diagnostic).with_source_code(miette::NamedSource::new(
            file.to_str().unwrap(),
            content.clone(),
        ));

        eprintln!("{:?}", report);
    }

    Ok(())
}

struct Code {
    source: Rc<str>,
}

impl miette::SourceCode for Code {
    fn read_span<'a>(
        &'a self,
        span: &miette::SourceSpan,
        _context_lines_before: usize,
        _context_lines_after: usize,
    ) -> Result<Box<dyn miette::SpanContents<'a> + 'a>, miette::MietteError> {
        Ok(Box::new(Content {
            source: self.source.clone(),
            span,
        }))
    }
}

struct Content<'a> {
    source: Rc<str>,
    span: &'a miette::SourceSpan,
}

impl Content<'_> {
    fn span(&self) -> std::ops::Range<usize> {
        let start = self.span.offset();
        let end = start + self.span.len();

        start..end
    }

    fn position(&self) -> (usize, usize) {
        let span = self.span();
        let mut line = 0usize;
        let mut column = 0usize;

        let mut chars = self.source.chars().enumerate().peekable();

        loop {
            let Some((idx, c)) = chars.next() else {
                break;
            };

            column += 1;

            match c {
                '\n' => {
                    line += 1;
                    column = 0;
                }
                '\r' if chars.peek().is_some_and(|(_, c)| c == &'\n') => continue,
                '\r' => {
                    line += 1;
                    column = 0;
                }
                _ => (),
            }

            if span.contains(&idx) {
                break;
            }
        }

        (line, column)
    }
}

impl<'a> miette::SpanContents<'a> for Content<'a> {
    fn data(&self) -> &'a [u8] {
        self.source[self.span()].as_bytes()
    }

    fn span(&self) -> &miette::SourceSpan {
        self.span
    }

    fn line(&self) -> usize {
        self.position().0
    }

    fn column(&self) -> usize {
        self.position().1
    }

    fn line_count(&self) -> usize {
        self.source[self.span()]
            .chars()
            .coalesce(|prev, curr| {
                if prev == '\r' && curr == '\n' {
                    Ok('\n')
                } else {
                    Err((prev, curr))
                }
            })
            .fold(0usize, |lines, c| {
                if ['\n', '\r'].contains(&c) {
                    lines + 1
                } else {
                    lines
                }
            })
    }
}
