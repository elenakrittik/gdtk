use gdtk_ast::{Visitor, ast};

use crate::declare_lint;

declare_lint!(
    UnnecessaryPass,
    code = "gdtk::redundancy::unnecessary_pass",
    severity = Advice
);

impl Visitor for UnnecessaryPass {
    fn visit_block(&mut self, block: &[ast::ASTStatement]) {
        for stmt in block {
            if stmt.is_pass() && block.len() > 2 {
                self.report("Unnecessary `pass`, this block contains other statements.", Some(&stmt.as_pass().unwrap().range));
                continue;
            }

            self.visit_statement(stmt);
        }
    }
}
