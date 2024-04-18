use gdtk_ast::Visitor;
use heck::{ToShoutySnakeCase, ToSnakeCase, ToTitleCase};
use miette::MietteDiagnostic;

pub struct IdentifierCasing(pub Vec<MietteDiagnostic>);

impl IdentifierCasing {
    pub fn report(&mut self, message: &'static str, range: std::ops::Range<usize>) {
        let diagnostic = miette::MietteDiagnostic::new(message)
            .with_code("gdtk::style::identifier_casing")
            .with_severity(miette::Severity::Advice)
            .with_label(miette::LabeledSpan::at(
                range,
                "This identifier is incorrectly cased.",
            ));

        self.0.push(diagnostic);
    }
}

impl Visitor for IdentifierCasing {
    fn visit_class(&mut self, class: &gdtk_ast::ASTClass) {
        let cased = class.identifier.as_identifier().unwrap().to_title_case();

        if cased != *class.identifier.as_identifier().unwrap() {
            self.report("Class name is not in title case.");
        }

        self.visit_block(class.body.as_slice());
    }

    fn visit_class_name_statement(&mut self, identifier: &gdtk_ast::ASTExprKind) {
        let cased = identifier.as_identifier().unwrap().to_title_case();

        if cased != *identifier.as_identifier().unwrap() {
            self.report("Class name is not in title case.");
        }
    }

    fn visit_enum_statement(&mut self, enum_: &gdtk_ast::ASTEnum) {
        if let Some(identifier) = &enum_.identifier {
            let cased = identifier.as_identifier().unwrap().to_title_case();

            if cased != *identifier.as_identifier().unwrap() {
                self.report("Enum name is not in title case.");
            }
        }

        self.visit_enum_variants(enum_.variants.as_slice());
    }

    fn visit_enum_variant(&mut self, variant: &gdtk_ast::ASTEnumVariant) {
        let cased = variant
            .identifier
            .as_identifier()
            .unwrap()
            .to_shouty_snake_case();

        if cased != *variant.identifier.as_identifier().unwrap() {
            self.report("Enum variant name is not in screaming snake case.");
        }
    }

    fn visit_func(&mut self, func: &gdtk_ast::ASTFunction) {
        if let Some(identifier) = &func.identifier {
            let cased = identifier.as_identifier().unwrap().to_snake_case();

            if cased != *identifier.as_identifier().unwrap() {
                self.report("Function name is not in snake case.");
            }
        }

        self.visit_parameters(func.parameters.as_slice());
        self.visit_block(func.body.as_slice());
    }

    fn visit_signal_statement(&mut self, signal: &gdtk_ast::ASTSignal) {
        let cased = signal.identifier.as_identifier().unwrap().to_snake_case();

        if cased != *signal.identifier.as_identifier().unwrap() {
            self.report("Signal name is not in snake case.");
        }

        if let Some(params) = &signal.parameters {
            self.visit_parameters(params.as_slice());
        }
    }

    fn visit_binding_variable(&mut self, variable: &gdtk_ast::ASTVariable) {
        let cased = variable.identifier.as_identifier().unwrap().to_snake_case();

        if cased != *variable.identifier.as_identifier().unwrap() {
            self.report("Binding name is not in snake case.");
        }
    }

    fn visit_any_variable(&mut self, variable: &gdtk_ast::ASTVariable) {
        let cased = variable.identifier.as_identifier().unwrap().to_snake_case();

        if cased != *variable.identifier.as_identifier().unwrap() {
            self.report("Variable name is not in snake case.");
        }
    }
}
