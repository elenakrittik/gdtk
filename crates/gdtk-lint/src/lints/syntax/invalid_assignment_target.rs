use diagnosis::{Diagnostic, Highlight, Severity};
use gdtk_gdscript_ast::{ast, visitor::walk_expr, Visitor};

crate::lint!(InvalidAssignmentTarget);

impl<'s> Visitor<'s> for InvalidAssignmentTarget<'s> {
    fn visit_binary_expr(
        &mut self,
        lhs: &'s ast::ASTExpr,
        op: &'s ast::ASTBinaryOp,
        rhs: &'s ast::ASTExpr,
        _span: &'s gdtk_span::Span,
    ) {
        if op.is_any_assignment() && !is_valid_assignment_target(lhs) {
            let mut diag = Diagnostic::new("Invalid assignment target.", Severity::Error)
                .with_code("invalid-assignment-target")
                .with_span(&lhs.span)
                .add_highlight(
                    Highlight::new(&rhs.span)
                        .with_message("..while trying to assign this expression"),
                )
                .add_highlight(
                    Highlight::new(&lhs.span).with_message("..to this target expression"),
                );

            if let Some((_, op, _)) = lhs.kind.as_binary_expr()
                && op.is_any_assignment()
            {
                diag = diag.add_help("Assignment chains are not valid syntax.");
            }

            self.0.push(diag);
        }

        walk_expr(self, lhs);
        walk_expr(self, rhs);
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
