pub type Span = std::ops::Range<usize>;

#[derive(Debug)]
pub struct Diagnostic {
    pub kind: DiagnosticKind,
    pub message: String,
    pub hint: Option<String>,
    pub span: Span,
}

#[derive(Debug)]
pub enum DiagnosticKind {
    // Info,
    Warning,
    Error,
}
