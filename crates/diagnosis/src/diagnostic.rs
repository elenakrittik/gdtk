pub type Span = std::ops::Range<usize>;

/// A diagnostic.
pub struct Diagnostic<'a> {
    /// The primary message of the diagnostic.
    pub message: &'a str,
    /// The severity of the diagnostic.
    pub severity: Severity,
    /// The code of the diagnostic.
    pub code: Option<&'a str>,
    /// The primary source span of the diagnostic.
    pub span: Option<&'a Span>,
    /// Additional labels attached to the diagnostic.
    pub labels: Vec<Label<'a>>,
    /// Additional help messages attached to the diagnostic.
    pub help: Vec<&'a str>,
}

impl<'a> Diagnostic<'a> {
    pub fn new(message: &'a str, severity: Severity) -> Self {
        Self {
            message,
            severity,
            code: None,
            span: None,
            labels: Vec::new(),
            help: Vec::new(),
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

    pub fn add_label(mut self, label: Label<'a>) -> Self {
        self.labels.push(label);
        self
    }

    pub fn add_help(mut self, help: &'a str) -> Self {
        self.help.push(help);
        self
    }
}

/// The severity of a diagnostic.
pub enum Severity {
    /// A critical error that prevents the program from doing job.
    Error,
    /// A warning that may impact the program's behavior in a non-fatal way.
    Warning,
    /// A general informational message.
    Advice,
}

/// A label attached to a diagnostic.
pub struct Label<'a> {
    /// The message of the label.
    pub message: &'a str,
    /// The source span of the label.
    pub span: &'a Span,
}

impl<'a> Label<'a> {
    pub fn new(message: &'a str, span: &'a Span) -> Self {
        Self { message, span }
    }
}
