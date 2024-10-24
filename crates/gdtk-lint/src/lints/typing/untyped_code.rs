use diagnosis::{Diagnostic, Highlight, Severity};
use gdtk_gdscript_ast::{
    ast,
    visitor::{walk_any_variable, walk_func},
    Visitor,
};

crate::lint!(UntypedCode);

impl<'s> Visitor<'s> for UntypedCode<'s> {
    fn visit_func(&mut self, func: &'s ast::ASTFunction<'s>) {
        if func.return_type.is_none() {
            self.0.push(
                Diagnostic::new("Missing return type.", Severity::Warning)
                    .with_code("untyped-code")
                    .with_span(&func.span)
                    .add_highlight(
                        Highlight::new(if let Some(ident) = &func.identifier {
                            &ident.span
                        } else {
                            &func.span
                        })
                        .with_message("This function is missing a return type annotation."),
                    ),
            );
        }

        walk_func(self, func);
    }

    // TODO: different messages for parameters/bindings
    fn visit_any_variable(&mut self, variable: &'s ast::ASTVariable<'s>) {
        if !(variable.infer_type || variable.typehint.is_some()) {
            self.0.push(
                Diagnostic::new("Missing static type.", Severity::Warning)
                    .with_code("untyped-code")
                    .with_span(&variable.identifier.span)
                    .add_highlight(
                        Highlight::new(&variable.identifier.span)
                        .with_message("This variable does not have a static type annotation nor has specified to infer the type.")
                    ),
            );
        }

        walk_any_variable(self, variable);
    }
}
