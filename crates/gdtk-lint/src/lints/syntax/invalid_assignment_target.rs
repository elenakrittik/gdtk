use gdtk_ast::{Visitor, ast};

crate::declare_lint!(
    InvalidAssignmentTarget,
    code = "gdtk::syntax::invalid_assignment_target",
    severity = Error
);

impl Visitor for InvalidAssignmentTarget {
    fn visit_binary_expr(
        &mut self,
        lhs: &ast::ASTExpr,
        op: &ast::ASTBinaryOp,
        rhs: &ast::ASTExpr,
        range: Option<&std::ops::Range<usize>>,
    ) {
        if op.is_any_assignment() && !is_valid_assignment_target(lhs) {
            self.report("Invalid assignment target.", lhs.range.as_ref());
        }

        self.visit_expr(lhs);
        self.visit_expr(rhs);
    }
}

fn is_valid_assignment_target(expr: &ast::ASTExpr) -> bool {
    match &expr.kind {
        ast::ASTExprKind::BinaryExpr(lhs, op, rhs) => {
            is_valid_assignment_target(&lhs)
            && op.is_property_access()
            && is_valid_assignment_target(&rhs)
        },
        ast::ASTExprKind::PostfixExpr(expr, op) => is_valid_assignment_target(&expr) && match op {
            ast::ASTPostfixOp::Subscript(_) => true,
            ast::ASTPostfixOp::Call(_)  => false,
        },
        ast::ASTExprKind::Identifier(_) => true,
        _ => false,
    }
}
