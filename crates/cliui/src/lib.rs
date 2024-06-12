//! Good-looking prompts and whatnot for your command-line applications.

#![feature(stmt_expr_attributes)]

pub use crate::error::{Error, Result};
pub use crate::prompt::Prompt;

mod error;
mod prompt;
