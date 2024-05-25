use diagnosis::{Diagnostic, Highlight, Severity};
use gdtk_gdscript_ast::{ast, Visitor};

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
                        .with_message("..in this function"),
                    ),
            );
        }
    }
}
