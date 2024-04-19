#[macro_export]
macro_rules! declare_lint {
    ($name:ident, code = $code:expr, severity = $severity:ident) => {
        pub struct $name(pub Vec<miette::MietteDiagnostic>);

        impl $name {
            pub fn report(&mut self, message: &'static str, range: Option<&std::ops::Range<usize>>) {
                let mut diagnostic = miette::MietteDiagnostic::new(message)
                    .with_code($code)
                    .with_severity(miette::Severity::$severity);

                if let Some(range) = range {
                    diagnostic = diagnostic
                        .with_label(miette::LabeledSpan::at(range.start..range.end, message));
                }

                self.0.push(diagnostic);
            }
        }
    }
}
