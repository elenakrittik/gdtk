use gdtk_ast::{ast, visitor, Visitor};

#[gdtk_macros::lint(
    message = "Self is not allowed in static functions.",
    code = "gdtk::syntax::self_in_static_func",
    severity = Error
)]
pub struct SelfInStaticFunc<'a> {
    current_func: Option<&'a ast::ASTFunction<'a>>,
}

impl<'a> Visitor<'a> for SelfInStaticFunc<'a> {
    fn visit_func(&mut self, func: &'a ast::ASTFunction<'a>) {
        let previous = self.current_func.replace(func);

        visitor::walk_func(self, func);

        self.current_func = previous;
    }

    fn visit_identifier_expr(&mut self, identifier: &str, span: &gdtk_span::Span) {
        if self.current_func.is_some_and(|func| func.kind.is_static()) && identifier == "self" {
            let report = Self::report()
                .and_label(miette::LabeledSpan::at(span.clone(), "`self` found here"));

            self.submit(report);
        }
    }
}
