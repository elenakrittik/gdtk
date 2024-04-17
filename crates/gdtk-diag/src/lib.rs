#[derive(Debug, Clone)]
pub struct Diagnostic {
    pub code: &'static str,
    pub message: &'static str,
    pub range: std::ops::Range<usize>,
    pub kind: DiagnosticKind,
}

#[derive(Debug, Clone)]
pub enum DiagnosticKind {
    // Info,
    Warning,
    Error,
}
