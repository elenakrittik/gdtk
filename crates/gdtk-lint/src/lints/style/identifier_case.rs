use diagnosis::{Diagnostic, Highlight, Severity};
use gdtk_ast::{
    ast,
    visitor::{walk_block, walk_enum_variants, walk_parameters},
    Visitor,
};

crate::lint!(IdentifierCase);

impl<'s> Visitor<'s> for IdentifierCase<'s> {
    fn visit_class(&mut self, class: &'s ast::ASTClassStmt) {
        let identifier = *class.identifier.kind.as_identifier().unwrap();
        let cased = is_upper_camel_case(identifier);

        if !cased {
            self.0.push(
                Diagnostic::new(
                    "Class names should be in UpperCamelCase.",
                    Severity::Warning,
                )
                .with_code("identifier-case")
                .with_span(&class.identifier.span)
                .add_highlight(Highlight::new(&class.identifier.span)),
            );
        }

        walk_block(self, class.body.as_slice());
    }

    fn visit_class_name_statement(&mut self, stmt: &'s ast::ASTClassNameStmt) {
        let identifier = *stmt.identifier.kind.as_identifier().unwrap();
        let cased = is_upper_camel_case(identifier);

        if !cased {
            self.0.push(
                Diagnostic::new(
                    "Class names should be in UpperCamelCase.",
                    Severity::Warning,
                )
                .with_code("identifier-case")
                .with_span(&stmt.identifier.span)
                .add_highlight(Highlight::new(&stmt.identifier.span)),
            );
        }
    }

    fn visit_enum_statement(&mut self, enum_: &'s ast::ASTEnumStmt) {
        if let Some(identifier) = &enum_.identifier {
            let ident = *identifier.kind.as_identifier().unwrap();
            let cased = is_upper_camel_case(ident);

            if !cased {
                self.0.push(
                    Diagnostic::new("Enum names should be in UpperCamelCase.", Severity::Warning)
                        .with_code("identifier-case")
                        .with_span(&identifier.span)
                        .add_highlight(Highlight::new(&identifier.span)),
                );
            }
        }

        walk_enum_variants(self, enum_.variants.as_slice());
    }

    fn visit_enum_variant(&mut self, variant: &'s ast::ASTEnumVariant) {
        let identifier = *variant.identifier.kind.as_identifier().unwrap();
        let cased = is_screaming_snake_case(identifier);

        if !cased {
            self.0.push(
                Diagnostic::new(
                    "Enum variant names should be in SCREAMING_SNAKE_CASE.",
                    Severity::Warning,
                )
                .with_code("identifier-case")
                .with_span(&variant.span)
                .add_highlight(Highlight::new(&variant.identifier.span)),
            );
        }
    }

    fn visit_func(&mut self, func: &'s ast::ASTFunction) {
        if let Some(identifier) = &func.identifier {
            let ident = *identifier.kind.as_identifier().unwrap();
            let cased = is_snake_case(ident);

            if !cased {
                self.0.push(
                    Diagnostic::new("Function names should be in snake_case.", Severity::Warning)
                        .with_code("identifier-case")
                        .with_span(&identifier.span)
                        .add_highlight(Highlight::new(&identifier.span)),
                );
            }
        }

        walk_parameters(self, func.parameters.as_slice());
        walk_block(self, func.body.as_slice());
    }

    fn visit_signal_statement(&mut self, signal: &'s ast::ASTSignalStmt) {
        let identifier = *signal.identifier.kind.as_identifier().unwrap();
        let cased = is_snake_case(identifier);

        if !cased {
            self.0.push(
                Diagnostic::new("Signal names should be in snake_case.", Severity::Warning)
                    .with_code("identifier-case")
                    .with_span(&signal.identifier.span)
                    .add_highlight(Highlight::new(&signal.identifier.span)),
            );
        }

        if let Some(params) = &signal.parameters {
            walk_parameters(self, params.as_slice());
        }
    }

    fn visit_any_variable(&mut self, variable: &'s ast::ASTVariable) {
        let identifier = *variable.identifier.kind.as_identifier().unwrap();

        let (cased, message) = variable_case_helper(identifier, variable);

        if !cased {
            self.0.push(
                Diagnostic::new(message, Severity::Warning)
                    .with_code("identifier-case")
                    .with_span(&variable.identifier.span)
                    .add_highlight(Highlight::new(&variable.identifier.span)),
            );
        }
    }
}

fn is_upper_camel_case(ident: &str) -> bool {
    for c in ident.chars() {
        if c == '_' {
            continue;
        }

        return c.is_ascii_uppercase();
    }

    // identifiers are always of length >= 1
    unreachable!()
}

fn is_snake_case(ident: &str) -> bool {
    ident
        .chars()
        .all(|c| c.is_ascii_digit() || c.is_ascii_lowercase() || c == '_')
}

fn is_screaming_snake_case(ident: &str) -> bool {
    ident
        .chars()
        .all(|c| c.is_ascii_digit() || c.is_ascii_uppercase() || c == '_')
}

fn variable_case_helper(identifier: &str, variable: &ast::ASTVariable) -> (bool, &'static str) {
    if let Some(value) = &variable.value
        && let Some((expr, op)) = value.kind.as_postfix_expr()
        && let Some(&"preload" | &"load") = expr.kind.as_identifier()
        && let Some(args) = op.kind.as_call()
        && let [arg] = args.as_slice()
        && let Some(s) = arg.kind.as_string()
        && (s.ends_with(".tscn") || s.ends_with(".gd"))
    {
        return (
            is_upper_camel_case(identifier),
            "Names of variables that `preload` or `load` scenes or scripts should be in UpperCamelCase."
        );
    }

    if variable.kind.is_constant() {
        return (
            is_screaming_snake_case(identifier),
            "Constant names should be in SCREAMING_SNAKE_CASE.",
        );
    }

    (
        is_snake_case(identifier),
        "Variable names should be in snake_case.",
    )
}
