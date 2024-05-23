use diagnosis::{Diagnostic, Highlight, Severity};
use gdtk_gdscript_ast::{ast, visitor::walk_block, Visitor};

crate::lint!(UnnecessaryPass);

impl<'s> Visitor<'s> for UnnecessaryPass<'s> {
    fn visit_block(&mut self, block: &'s [ast::ASTStatement<'s>]) {
        for stmt in block.iter().skip(1) {
            if let Some(stmt) = stmt.as_pass() {
                self.0.push(
                    Diagnostic::new("Unnecessary `pass`.", Severity::Warning)
                        .with_span(&stmt.span)
                        .with_code("unnecessary-pass")
                        .add_highlight(
                            Highlight::new(&stmt.span).with_message("`pass` found here"),
                        ),
                );
            }
        }

        walk_block(self, block);
    }
}
