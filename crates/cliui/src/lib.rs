//! Good-looking prompts and whatnot for your command-line applications.

#![feature(type_changing_struct_update, int_roundings)]

pub use crate::error::{Error, Result};
pub use crate::prompt::{Action, Key, Prompt};

mod error;
pub mod prompt;
