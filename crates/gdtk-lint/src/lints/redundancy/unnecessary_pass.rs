use gdtk_ast::{ast, Visitor};

use crate::declare_lint;

declare_lint!(
    UnnecessaryPass,
    code = "gdtk::redundancy::unnecessary_pass",
    severity = Advice
);

impl Visitor for UnnecessaryPass {
    fn visit_block(&mut self, block: &[ast::ASTStatement]) {
        for stmt in block {
            if let Some(stmt) = stmt.as_pass()
                && block.len() > 1
            {
                self.report(
                    "Unnecessary `pass`, this block is not empty.",
                    Some(&stmt.range),
                );
                continue;
            }

            self.visit_statement(stmt);
        }
    }
}
