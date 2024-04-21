use gdtk_ast::{ast, Visitor};

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
        _range: Option<&std::ops::Range<usize>>,
    ) {
        if op.is_any_assignment() && !is_valid_assignment_target(lhs) {
            self.report("Invalid assignment target.", lhs.range.as_ref());
        }

        self.visit_expr(lhs);
        self.visit_expr(rhs);
    }
}

fn is_valid_assignment_target(expr: &ast::ASTExpr) -> bool {
    fn is_valid_inner_target(expr: &ast::ASTExpr) -> bool {
        match &expr.kind {
            ast::ASTExprKind::BinaryExpr(lhs, op, rhs) => {
                is_valid_inner_target(lhs) && op.is_property_access() && is_valid_inner_target(rhs)
            }
            ast::ASTExprKind::PostfixExpr(expr, op) => {
                is_valid_inner_target(expr)
                    && match &op.kind {
                        ast::ASTPostfixOpKind::Subscript(_) => true,
                        ast::ASTPostfixOpKind::Call(_) => true,
                    }
            }
            ast::ASTExprKind::Identifier(_) => true,
            _ => false,
        }
    }

    // `get_people()[name] = person` is valid, but `get_people() = { name: person }` is not
    if expr
        .kind
        .as_postfix_expr()
        .is_some_and(|(_, op)| op.kind.as_call().is_some())
    {
        false
    } else {
        is_valid_inner_target(expr)
    }
}
