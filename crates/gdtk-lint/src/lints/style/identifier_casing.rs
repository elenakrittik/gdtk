use gdtk_ast::Visitor;
use heck::{ToTitleCase, ToSnakeCase, ToShoutySnakeCase};

#[derive(Debug, thiserror::Error)]
pub enum IdentifierCasingErrorKind {
    #[error("Class name is not in title case.")]
    ClassName,
    #[error("Enum name is not in title case.")]
    EnumName,
    #[error("Enum variant name is not in screaming snake case.")]
    EnumVariantName,
    #[error("Function name is not in snake case.")]
    FunctionName,
    #[error("Signal name is not in snake case.")]
    SignalName,
    #[error("Variable name is not in snake case.")]
    VariableName,
    #[error("Constant name is not in screaming snake case.")]
    ConstName,
    #[error("Binding name is not in snake case.")]
    BindingName,
}

#[derive(Debug, thiserror::Error, miette::Diagnostic)]
#[error("{error_kind}")]
pub struct IdentifierCasingError {
    pub error_kind: IdentifierCasingErrorKind,
}

pub struct IdentifierCasing {
    pub diagnostics: Vec<Box<dyn miette::Diagnostic>>,
}

impl IdentifierCasing {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self { diagnostics: vec![] }
    }

    fn report(&mut self, kind: IdentifierCasingErrorKind) {
        self.diagnostics.push(Box::new(IdentifierCasingError { error_kind: kind }));
    }
}

impl Visitor for IdentifierCasing {
    fn visit_class(&mut self, class: &gdtk_ast::ASTClass) {
        let cased = class.identifier.to_title_case();

        if cased != class.identifier {
            self.report(IdentifierCasingErrorKind::ClassName);
        }

        self.visit_block(class.body.as_slice());
    }

    fn visit_class_name_statement(&mut self, identifier: &str) {
        let cased = identifier.to_title_case();

        if cased != identifier {
            self.report(IdentifierCasingErrorKind::ClassName);
        }
    }

    fn visit_enum_statement(&mut self, enum_: &gdtk_ast::ASTEnum) {
        if let Some(identifier) = enum_.identifier {
            let cased = identifier.to_title_case();

            if cased != identifier {
                self.report(IdentifierCasingErrorKind::EnumName);
            }
        }

        self.visit_enum_variants(enum_.variants.as_slice());
    }

    fn visit_enum_variant(&mut self, variant: &gdtk_ast::ASTEnumVariant) {
        let cased = variant.identifier.to_shouty_snake_case();

        if cased != variant.identifier {
            self.report(IdentifierCasingErrorKind::EnumVariantName);
        }
    }

    fn visit_func(&mut self, func: &gdtk_ast::ASTFunction) {
        if let Some(identifier) = func.identifier {
            let cased = identifier.to_snake_case();

            if cased != identifier {
                self.report(IdentifierCasingErrorKind::FunctionName);
            }
        }

        self.visit_parameters(func.parameters.as_slice());
        self.visit_block(func.body.as_slice());
    }

    fn visit_signal_statement(&mut self, signal: &gdtk_ast::ASTSignal) {
        let cased = signal.identifier.to_snake_case();

        if cased != signal.identifier {
            self.report(IdentifierCasingErrorKind::SignalName);
        }

        if let Some(params) = &signal.parameters {
            self.visit_parameters(params.as_slice());
        }
    }

    fn visit_binding_variable(&mut self, variable: &gdtk_ast::ASTVariable) {
        let cased = variable.identifier.to_snake_case();

        if cased != variable.identifier {
            self.report(IdentifierCasingErrorKind::BindingName);
        }
    }

    fn visit_const_variable(&mut self, variable: &gdtk_ast::ast::ASTVariable) {
        let cased = variable.identifier.to_shouty_snake_case();

        if cased != variable.identifier {
            self.report(IdentifierCasingErrorKind::ConstName);
        }
    }

    fn visit_any_variable(&mut self, variable: &gdtk_ast::ASTVariable) {
        let cased = variable.identifier.to_snake_case();

        if cased != variable.identifier {
            self.report(IdentifierCasingErrorKind::VariableName);
        }
    }
}
