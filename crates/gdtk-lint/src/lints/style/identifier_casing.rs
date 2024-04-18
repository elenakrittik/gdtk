use gdtk_ast::Visitor;
use heck::{ToShoutySnakeCase, ToSnakeCase, ToTitleCase};
use miette::MietteDiagnostic;

pub struct IdentifierCasing(pub Vec<MietteDiagnostic>);

impl IdentifierCasing {
    pub fn report(&mut self, message: &'static str, range: Option<&std::ops::Range<usize>>) {
        let mut diagnostic = miette::MietteDiagnostic::new(message)
            .with_code("gdtk::style::identifier_casing")
            .with_severity(miette::Severity::Advice);

        if let Some(range) = range {
            diagnostic =
                diagnostic.with_label(miette::LabeledSpan::at(range.start..range.end, message));
        }

        self.0.push(diagnostic);
    }
}

impl Visitor for IdentifierCasing {
    fn visit_class(&mut self, class: &gdtk_ast::ASTClassStmt) {
        let identifier = class.identifier.kind.as_identifier().unwrap();
        let cased = identifier.to_title_case();

        if cased != *identifier {
            self.report(
                "Class name is not in title case.",
                class.identifier.range.as_ref(),
            );
        }

        self.visit_block(class.body.as_slice());
    }

    fn visit_class_name_statement(&mut self, stmt: &gdtk_ast::ASTClassNameStmt) {
        let identifier = stmt.identifier.kind.as_identifier().unwrap();
        let cased = identifier.to_title_case();

        eprintln!("iwasthere");

        if cased != *identifier {
            self.report(
                "Class name is not in title case.",
                stmt.identifier.range.as_ref(),
            );
        }
    }

    fn visit_enum_statement(&mut self, enum_: &gdtk_ast::ASTEnumStmt) {
        if let Some(identifier) = &enum_.identifier {
            let ident = identifier.kind.as_identifier().unwrap();
            let cased = ident.to_title_case();

            if cased != *ident {
                self.report("Enum name is not in title case.", identifier.range.as_ref());
            }
        }

        self.visit_enum_variants(enum_.variants.as_slice());
    }

    fn visit_enum_variant(&mut self, variant: &gdtk_ast::ASTEnumVariant) {
        let identifier = variant.identifier.kind.as_identifier().unwrap();
        let cased = identifier.to_shouty_snake_case();

        if cased != *identifier {
            self.report(
                "Enum variant name is not in screaming snake case.",
                variant.identifier.range.as_ref(),
            );
        }
    }

    fn visit_func(&mut self, func: &gdtk_ast::ASTFunction) {
        if let Some(identifier) = &func.identifier {
            let ident = identifier.kind.as_identifier().unwrap();
            let cased = ident.to_snake_case();

            if cased != *ident {
                self.report(
                    "Function name is not in snake case.",
                    identifier.range.as_ref(),
                );
            }
        }

        self.visit_parameters(func.parameters.as_slice());
        self.visit_block(func.body.as_slice());
    }

    fn visit_signal_statement(&mut self, signal: &gdtk_ast::ASTSignalStmt) {
        let identifier = signal.identifier.kind.as_identifier().unwrap();
        let cased = identifier.to_snake_case();

        if cased != *identifier {
            self.report(
                "Signal name is not in snake case.",
                signal.identifier.range.as_ref(),
            );
        }

        if let Some(params) = &signal.parameters {
            self.visit_parameters(params.as_slice());
        }
    }

    fn visit_binding_variable(&mut self, variable: &gdtk_ast::ASTVariable) {
        let identifier = variable.identifier.kind.as_identifier().unwrap();
        let cased = identifier.to_snake_case();

        if cased != *identifier {
            self.report(
                "Binding name is not in snake case.",
                variable.identifier.range.as_ref(),
            );
        }
    }

    fn visit_any_variable(&mut self, variable: &gdtk_ast::ASTVariable) {
        let identifier = variable.identifier.kind.as_identifier().unwrap();
        let cased = identifier.to_snake_case();

        if cased != *identifier {
            self.report(
                "Variable name is not in snake case.",
                variable.identifier.range.as_ref(),
            );
        }
    }
}
