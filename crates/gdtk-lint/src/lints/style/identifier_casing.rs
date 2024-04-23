use diagnosis::{Diagnostic, Severity};
use gdtk_ast::{ast, Visitor};
use heck::{ToShoutySnakeCase, ToSnakeCase, ToUpperCamelCase};

crate::lint!(IdentifierCasing);

impl<'s> Visitor<'s> for IdentifierCasing<'s> {
    fn visit_class(&mut self, class: &'s ast::ASTClassStmt) {
        let identifier = class.identifier.kind.as_identifier().unwrap();
        let cased = identifier.to_upper_camel_case();

        if cased != *identifier {
            self.0.push(
                Diagnostic::new("Class names should be in UpperCamelCase.", Severity::Advice)
                    .with_span(&class.identifier.span),
            );
        }

        self.visit_block(class.body.as_slice());
    }

    fn visit_class_name_statement(&mut self, stmt: &'s ast::ASTClassNameStmt) {
        let identifier = stmt.identifier.kind.as_identifier().unwrap();
        let cased = identifier.to_upper_camel_case();

        if cased != *identifier {
            self.0.push(
                Diagnostic::new("Class names should be in UpperCamelCase.", Severity::Advice)
                    .with_span(&stmt.identifier.span),
            );
        }
    }

    fn visit_enum_statement(&mut self, enum_: &'s ast::ASTEnumStmt) {
        if let Some(identifier) = &enum_.identifier {
            let ident = identifier.kind.as_identifier().unwrap();
            let cased = ident.to_upper_camel_case();

            if cased != *ident {
                self.0.push(
                    Diagnostic::new("Enum names should be in UpperCamelCase.", Severity::Advice)
                        .with_span(&identifier.span),
                );
            }
        }

        self.visit_enum_variants(enum_.variants.as_slice());
    }

    fn visit_enum_variant(&mut self, variant: &'s ast::ASTEnumVariant) {
        let identifier = variant.identifier.kind.as_identifier().unwrap();
        let cased = identifier.to_shouty_snake_case();

        if cased != *identifier {
            self.0.push(
                Diagnostic::new(
                    "Enum variant names should be in SCREAMING_SNAKE_CASE.",
                    Severity::Advice,
                )
                .with_span(&variant.span),
            );
        }
    }

    fn visit_func(&mut self, func: &'s ast::ASTFunction) {
        if let Some(identifier) = &func.identifier {
            let ident = identifier.kind.as_identifier().unwrap();
            let cased = ident.to_snake_case();

            if cased != *ident {
                self.0.push(
                    Diagnostic::new("Function names should be in snake_case.", Severity::Advice)
                        .with_span(&identifier.span),
                );
            }
        }

        self.visit_parameters(func.parameters.as_slice());
        self.visit_block(func.body.as_slice());
    }

    fn visit_signal_statement(&mut self, signal: &'s ast::ASTSignalStmt) {
        let identifier = signal.identifier.kind.as_identifier().unwrap();
        let cased = identifier.to_snake_case();

        if cased != *identifier {
            self.0.push(
                Diagnostic::new("Signal names should be in snake_case.", Severity::Advice)
                    .with_span(&signal.identifier.span),
            );
        }

        if let Some(params) = &signal.parameters {
            self.visit_parameters(params.as_slice());
        }
    }

    fn visit_any_variable(&mut self, variable: &'s ast::ASTVariable) {
        let identifier = variable.identifier.kind.as_identifier().unwrap();
        let cased = identifier.to_snake_case();

        if cased != *identifier {
            self.0.push(
                Diagnostic::new("Variable names should be in snake_case.", Severity::Advice)
                    .with_span(&variable.identifier.span),
            );
        }
    }
}
