use crate::Span;

/// A diagnostic.
#[derive(Debug)]
pub struct Diagnostic<'a> {
    /// The primary message of the diagnostic.
    pub message: &'a str,
    /// The severity of the diagnostic.
    pub severity: Severity<'a>,
    /// The code of the diagnostic.
    pub code: Option<&'a str>,
    /// The primary source span of the diagnostic.
    pub span: Option<&'a Span>,
    /// Additional highlights attached to the diagnostic.
    pub highlights: Vec<Highlight<'a>>,
    /// Additional help messages attached to the diagnostic.
    pub help_messages: Vec<&'a str>,
}

impl<'a> Diagnostic<'a> {
    pub fn new(message: &'a str, severity: Severity<'a>) -> Self {
        Self {
            message,
            severity,
            code: None,
            span: None,
            highlights: vec![],
            help_messages: vec![],
        }
    }

    pub fn with_code(mut self, code: &'a str) -> Self {
        self.code = Some(code);
        self
    }

    pub fn with_span(mut self, span: &'a Span) -> Self {
        self.span = Some(span);
        self
    }

    pub fn add_highlight(mut self, highlight: Highlight<'a>) -> Self {
        self.highlights.push(highlight);
        self
    }

    pub fn add_help(mut self, help: &'a str) -> Self {
        self.help_messages.push(help);
        self
    }
}

/// The severity of a diagnostic.
#[derive(Debug)]
pub enum Severity<'a> {
    /// A critical error that prevents the program from doing job.
    Error,
    /// A warning that may impact the program's behavior in a non-fatal way.
    Warning,
    /// A custom message kind.
    Custom(&'a str),
}

/// A hightlight attached to a diagnostic.
#[derive(Debug)]
pub struct Highlight<'a> {
    /// The source span of the highlight.
    pub span: &'a Span,
    /// The message of the hightlight.
    pub message: Option<&'a str>,
}

impl<'a> Highlight<'a> {
    pub fn new(span: &'a Span) -> Self {
        Self {
            span,
            message: None,
        }
    }

    pub fn with_message(mut self, message: &'a str) -> Self {
        self.message = Some(message);
        self
    }
}
