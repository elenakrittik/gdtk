//! An opinionated but flexible command-line application framework.
//!
//! The library consists of just handful of types that overarch your
//! application's structure, but the details of parsing and execution
//! are left up to you.
//!
//! The flow goes like this:
//! - Define your command structs/enums. Each command must implement
//!   the [Command] trait.
//! - Parse the root command from the environment using [Command::from_env].
//! - Run the root command (i.e. your whole application) using [Command::run].
//!
//! ## Examples
//!
//! ```no_run
//! use tapcli::*;
//!
//! struct Cli {
//!     name: String,
//! }
//!
//! impl Command for Cli {
//!     fn parse(parser: &mut Parser) -> Result<Self, Self::Error> {
//!         let name = match parser.next().unwrap() {
//!             Arg::Value(name) => name,
//!             other => panic!("Expected a value, got {:?}", other),
//!         };
//!
//!         Ok(Self { name })
//!     }
//!
//!     fn run(self) -> Result<Self::Output, Self::Error> {
//!         eprintln!("Hello, {}!", self.name);
//!
//!         Ok(())
//!     }
//! }
//!
//! fn app() -> Result<(), Box<dyn std::error::Error>> {
//!     let cli = Cli::from_env()?;
//!
//!     cli.run()?;
//!
//!     Ok(())
//! }
//! ```

#![feature(
    associated_type_defaults,
    never_type,
    assert_matches,
    impl_trait_in_assoc_type
)]
#![warn(missing_docs, missing_debug_implementations)]

mod arg;
mod command;
mod parser;

pub use crate::arg::{Arg, ArgRef};
pub use crate::command::Command;
pub use crate::parser::Parser;
