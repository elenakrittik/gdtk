//! Good-looking prompts and whatnot for your command-line applications.

#![feature(type_changing_struct_update)]

pub use crate::error::{Error, Result};
pub use crate::prompt::{Prompt, PromptBuilder};

mod error;
mod prompt;
