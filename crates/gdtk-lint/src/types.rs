use std::{collections::BTreeMap, sync::Arc};

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct LintQuery<'a> {
    pub identifier: &'a str,
    pub query: &'a str,
    pub args: BTreeMap<Arc<str>, trustfall::FieldValue>,
}
