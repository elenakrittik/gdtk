use diagnosis::{Diagnostic, Label, Severity};
use gdtk_ast::{ast, visitor, Visitor};

#[derive(Default)]
pub struct SelfInStaticFunc<'s> {
    pub diagnostics: Vec<Diagnostic<'s>>,
    current_func: Option<&'s ast::ASTFunction<'s>>,
}

impl<'s> Visitor<'s> for SelfInStaticFunc<'s> {
    fn visit_func(&mut self, func: &'s ast::ASTFunction<'s>) {
        let previous = self.current_func.replace(func);

        visitor::walk_func(self, func);

        self.current_func = previous;
    }

    fn visit_identifier_expr(&mut self, identifier: &'s str, span: &'s gdtk_span::Span) {
        if self.current_func.is_some_and(|func| func.kind.is_static()) && identifier == "self" {
            self.diagnostics.push(
                Diagnostic::new(
                    "`self` cannot be used in `static` functions",
                    Severity::Error,
                )
                .add_label(Label::new("`self` found here", span)),
            );
        }
    }
}
