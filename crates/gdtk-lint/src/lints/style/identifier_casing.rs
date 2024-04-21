use gdtk_ast::{ast, Visitor};
use heck::{ToShoutySnakeCase, ToSnakeCase, ToTitleCase};

#[gdtk_macros::lint(
    message = "Identifier is incorrectly cased.",
    code = "gdtk::style::identifier_casing",
    severity = Advice
)]
pub struct IdentifierCasing {}

impl Visitor<'_> for IdentifierCasing {
    fn visit_class(&mut self, class: &ast::ASTClassStmt) {
        let identifier = class.identifier.kind.as_identifier().unwrap();
        let cased = identifier.to_title_case();

        if cased != *identifier {
            let report = Self::report().and_label(miette::LabeledSpan::at(
                class.identifier.range.clone(),
                "Class names should be in PascalCase.",
            ));

            self.submit(report);
        }

        self.visit_block(class.body.as_slice());
    }

    fn visit_class_name_statement(&mut self, stmt: &ast::ASTClassNameStmt) {
        let identifier = stmt.identifier.kind.as_identifier().unwrap();
        let cased = identifier.to_title_case();

        if cased != *identifier {
            let report = Self::report().and_label(miette::LabeledSpan::at(
                stmt.identifier.range.clone(),
                "Class name is not in title case.",
            ));

            self.submit(report);
        }
    }

    fn visit_enum_statement(&mut self, enum_: &ast::ASTEnumStmt) {
        if let Some(identifier) = &enum_.identifier {
            let ident = identifier.kind.as_identifier().unwrap();
            let cased = ident.to_title_case();

            if cased != *ident {
                let report = Self::report().and_label(miette::LabeledSpan::at(
                    identifier.range.clone(),
                    "Enum name is not in title case.",
                ));

                self.submit(report);
            }
        }

        self.visit_enum_variants(enum_.variants.as_slice());
    }

    fn visit_enum_variant(&mut self, variant: &ast::ASTEnumVariant) {
        let identifier = variant.identifier.kind.as_identifier().unwrap();
        let cased = identifier.to_shouty_snake_case();

        if cased != *identifier {
            let report = Self::report().and_label(miette::LabeledSpan::at(
                variant.identifier.range.clone(),
                "Enum variant name is not in screaming snake case.",
            ));

            self.submit(report);
        }
    }

    fn visit_func(&mut self, func: &ast::ASTFunction) {
        if let Some(identifier) = &func.identifier {
            let ident = identifier.kind.as_identifier().unwrap();
            let cased = ident.to_snake_case();

            if cased != *ident {
                let report = Self::report().and_label(miette::LabeledSpan::at(
                    identifier.range.clone(),
                    "Function name is not in snake case.",
                ));

                self.submit(report);
            }
        }

        self.visit_parameters(func.parameters.as_slice());
        self.visit_block(func.body.as_slice());
    }

    fn visit_signal_statement(&mut self, signal: &ast::ASTSignalStmt) {
        let identifier = signal.identifier.kind.as_identifier().unwrap();
        let cased = identifier.to_snake_case();

        if cased != *identifier {
            let report = Self::report().and_label(miette::LabeledSpan::at(
                signal.identifier.range.clone(),
                "Signal name is not in snake case.",
            ));

            self.submit(report);
        }

        if let Some(params) = &signal.parameters {
            self.visit_parameters(params.as_slice());
        }
    }

    fn visit_binding_variable(&mut self, variable: &ast::ASTVariable) {
        let identifier = variable.identifier.kind.as_identifier().unwrap();
        let cased = identifier.to_snake_case();

        if cased != *identifier {
            let report = Self::report().and_label(miette::LabeledSpan::at(
                variable.identifier.range.clone(),
                "Binding name is not in snake case.",
            ));

            self.submit(report);
        }
    }

    fn visit_any_variable(&mut self, variable: &ast::ASTVariable) {
        let identifier = variable.identifier.kind.as_identifier().unwrap();
        let cased = identifier.to_snake_case();

        if cased != *identifier {
            let report = Self::report().and_label(miette::LabeledSpan::at(
                variable.identifier.range.clone(),
                "Variable name is not in snake case.",
            ));

            self.submit(report);
        }
    }
}
