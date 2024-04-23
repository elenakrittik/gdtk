use diagnosis::{Diagnostic, Label, Severity};
use gdtk_ast::{ast, visitor::walk_block, Visitor};

crate::lint!(UnnecessaryPass);

impl<'s> Visitor<'s> for UnnecessaryPass<'s> {
    fn visit_block(&mut self, block: &'s [ast::ASTStatement<'s>]) {
        for stmt in block.iter().skip(1) {
            if let Some(stmt) = stmt.as_pass() {
                self.0.push(
                    Diagnostic::new("Unnecessary `pass`.", Severity::Warning)
                        .with_span(&stmt.span)
                        .with_code("unnecessary-pass")
                        .add_label(Label::new("`pass` found here", &stmt.span)),
                );
            }
        }

        walk_block(self, block);
    }
}
