#[macro_export]
macro_rules! declare_lint {
    ($name:ident, code = $code:expr, severity = $severity:ident) => {
        pub struct $name(pub Vec<miette::MietteDiagnostic>);

        impl $name {
            pub fn report(
                &mut self,
                message: &'static str,
                range: &std::ops::Range<usize>,
            ) -> miette::MietteDiagnostic {
                miette::MietteDiagnostic::new(message)
                    .with_code($code)
                    .with_severity(miette::Severity::$severity)
                    .with_label(miette::LabeledSpan::at(range.start..range.end, message))
            }

            pub fn immediate_report(
                &mut self,
                message: &'static str,
                range: &std::ops::Range<usize>,
            ) {
                let report = self.report(message, range);
                self.submit(report);
            }

            pub fn submit(&mut self, report: miette::MietteDiagnostic) {
                self.0.push(report)
            }
        }
    };
}
