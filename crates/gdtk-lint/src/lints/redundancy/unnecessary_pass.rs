use gdtk_ast::{ast, Visitor};

#[gdtk_macros::lint(
    message = "Unnecessary `pass`, this block is not empty.",
    code = "gdtk::redundancy::unnecessary_pass",
    severity = Advice
)]
pub struct UnnecessaryPass {}

impl Visitor<'_> for UnnecessaryPass {
    fn visit_block(&mut self, block: &[ast::ASTStatement]) {
        for stmt in block.iter().skip(1) {
            if let Some(stmt) = stmt.as_pass()
                && block.len() > 1
            {
                let report = Self::report().and_label(miette::LabeledSpan::at(
                    stmt.range.clone(),
                    "`pass` found here",
                ));

                self.submit(report);

                continue;
            }

            self.visit_statement(stmt);
        }
    }
}
