use diagnosis::{Diagnostic, Label};
use gdtk_ast::{ast, Visitor};

crate::lint!(UnnecessaryPass);

impl<'s> Visitor<'s> for UnnecessaryPass<'s> {
    fn visit_block(&mut self, block: &'s [ast::ASTStatement]) {
        for stmt in block.iter().skip(1) {
            if let Some(stmt) = stmt.as_pass()
                && block.len() > 1
            {
                self.0.push(
                    Diagnostic::new("Unnecessary `pass`.", diagnosis::Severity::Advice)
                        .with_span(&stmt.span)
                        .add_label(Label::new("`pass` found here", &stmt.span)),
                );

                continue;
            }

            self.visit_statement(stmt);
        }
    }
}
