use gdtk_ast::Visitor;
use gdtk_diag::{Diagnostic, DiagnosticKind};
use heck::{ToTitleCase, ToSnakeCase, ToShoutySnakeCase};

pub struct IdentifierCasing(Vec<Diagnostic>);

impl IdentifierCasing {
    pub fn report(&mut self, message: &'static str) {
        self.0.push(Diagnostic {
            code: "S001",
            message,
            range: 0..0, // TODO
            kind: DiagnosticKind::Warning,
        });
    }
}

impl Visitor for IdentifierCasing {
    fn visit_class(&mut self, class: &gdtk_ast::ASTClass) {
        let cased = class.identifier.to_title_case();

        if &cased != class.identifier {
            self.report("Class name is not in title case.");
        }

        self.visit_block(class.body.as_slice());
    }

    fn visit_class_name_statement(&mut self, identifier: &str) {
        let cased = identifier.to_title_case();

        if &cased != identifier {
            eprintln!("S001: Class name is not in title case.");
        }
    }

    fn visit_enum_statement(&mut self, enum_: &gdtk_ast::ASTEnum) {
        if let Some(identifier) = enum_.identifier {
            let cased = identifier.to_title_case();

            if &cased != identifier {
                eprintln!("S001: Enum name is not in title case.");
            }
        }

        self.visit_enum_variants(enum_.variants.as_slice());
    }

    fn visit_enum_variant(&mut self, variant: &gdtk_ast::ASTEnumVariant) {
        let cased = variant.identifier.to_shouty_snake_case();

        if &cased != variant.identifier {
            eprintln!("S001: Enum variant name is not in screaming snake case.");
        }
    }

    fn visit_func(&mut self, func: &gdtk_ast::ASTFunction) {
        if let Some(identifier) = func.identifier {
            let cased = identifier.to_snake_case();

            if &cased != identifier {
                eprintln!("S001: Function name is not in snake case.");
            }
        }

        self.visit_parameters(func.parameters.as_slice());
        self.visit_block(func.body.as_slice());
    }

    fn visit_signal_statement(&mut self, signal: &gdtk_ast::ASTSignal) {
        let cased = signal.identifier.to_snake_case();

        if &cased != signal.identifier {
            eprintln!("S001: Signal name is not in snake case.");
        }

        if let Some(params) = &signal.parameters {
            self.visit_parameters(params.as_slice());
        }
    }

    fn visit_binding_variable(&mut self, variable: &gdtk_ast::ASTVariable) {
        let cased = variable.identifier.to_snake_case();

        if &cased != variable.identifier {
            eprintln!("S001: Binding name is not in snake case.");
        }
    }

    fn visit_any_variable(&mut self, variable: &gdtk_ast::ASTVariable) {
        let cased = variable.identifier.to_snake_case();

        if &cased != variable.identifier {
            eprintln!("S001: Variable name is not in snake case.");
        }
    }
}
