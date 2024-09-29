//! Good-looking prompts and whatnot for your command-line applications.

#![feature(decl_macro, type_changing_struct_update, int_roundings)]

pub use crate::display::StateDisplay;
pub use crate::error::{Error, Result};
pub use crate::prompt::{Action, Key, Prompt};

mod display;
mod error;
mod prompt;
