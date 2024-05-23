use diagnosis::{Diagnostic, Highlight, Severity};
use gdtk_gdscript_ast::{ast, visitor::walk_expr, Visitor};

crate::lint!(StandaloneExpression);

impl<'s> Visitor<'s> for StandaloneExpression<'s> {
    fn visit_expr_stmt(&mut self, expr: &'s ast::ASTExpr<'s>) {
        walk_expr(self, expr);

        if let Some((_, op)) = expr.kind.as_postfix_expr()
            && op.kind.is_call()
        {
            return;
        }

        if let Some((_, op, _)) = expr.kind.as_binary_expr()
            && op.is_any_assignment()
        {
            return;
        }

        self.0.push(
            Diagnostic::new("Standalone expression.", Severity::Warning)
                .with_span(&expr.span)
                .with_code("standalone-expression")
                .add_highlight(
                    Highlight::new(&expr.span).with_message("standalone expression found here"),
                ),
        );
    }
}
