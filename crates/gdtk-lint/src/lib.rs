use std::{collections::BTreeMap, sync::Arc};

use gdtk_gdscript_trustfall_adapter::GDScriptAdapter;
use trustfall::{execute_query, FieldValue, Schema};
use gdtk_ast::poor::ASTFile;

const QUERIES: &[&str] = &[
    include_str!("lints/non-pascalcase-class-name.ron"),
];

pub fn run_lints(file: &ASTFile) {
    let adapter = Arc::new(GDScriptAdapter::new(file));
    let schema = Schema::parse(include_str!("schema.graphql")).unwrap();
    let mut variables: BTreeMap<Arc<str>, FieldValue> = BTreeMap::new();
    variables.insert("regex".into(), "^[A-Z].*".into());
    let result = execute_query(&schema, adapter, query, variables).unwrap();
    let result = result.collect::<Vec<_>>();

    println!("{:#?}", result);
}
