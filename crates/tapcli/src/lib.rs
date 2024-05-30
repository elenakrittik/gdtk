//! A sane command-line application framework.

#![feature(associated_type_defaults, never_type, assert_matches)]
#![forbid(unsafe_code)]
#![warn(missing_docs, missing_debug_implementations, elided_lifetimes_in_paths)]

mod arg;
mod command;
mod parser;

pub use crate::arg::{Arg, ArgRef};
pub use crate::command::Command;
pub use crate::parser::Parser;
