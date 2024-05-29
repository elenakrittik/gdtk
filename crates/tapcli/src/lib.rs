//! A sane command-line application framework.

#![feature(associated_type_defaults, never_type, assert_matches)]
#![forbid(unsafe_code)]
#![warn(missing_docs, missing_debug_implementations, elided_lifetimes_in_paths)]

mod lexopt;

pub use lexopt::Arg;
pub use lexopt::Parser;

/// A command.
pub trait Command: Sized {
    /// The result of running the command.
    type Output = ();
    /// The error type for the command.
    type Error = !;

    /// Parse the command from the environment. Commonly used to parse
    /// the root command.
    fn from_env() -> Result<Self, Self::Error> {
        let mut parser = Parser::from_env();

        Self::parse(&mut parser)
    }

    /// Parse the command.
    fn parse(parser: &mut Parser) -> Result<Self, Self::Error>;

    /// Run the command.
    fn run(self) -> Result<Self::Output, Self::Error>;
}
