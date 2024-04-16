#![allow(incomplete_features)]
#![feature(deref_patterns)]

use std::sync::Arc;

mod adapter;
mod builtin;
mod types;

pub fn run_builtin_lints(file: gdtk_ast::poor::ASTFile<'_>) {
    let builtins = crate::builtin::get_builtin_lints();

    let adapter = Arc::new(crate::adapter::GDScriptAdapter::new(&file));
    let schema = trustfall::Schema::parse(include_str!("../schema.graphql")).unwrap();

    for lint in builtins {
        let result = trustfall::execute_query(&schema, adapter.clone(), lint.query, lint.args)
            .unwrap()
            .collect::<Vec<_>>();

        eprintln!("Result of running lint {}: {:?}", lint.identifier, result);
    }
}
