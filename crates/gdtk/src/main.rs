#![deny(clippy::disallowed_types)]
#![feature(let_chains, decl_macro, impl_trait_in_assoc_type)]
#![feature(generic_arg_infer)]

use tapcli::Command;

use crate::utils::setup_tracing;

pub mod cli;
pub mod utils;

fn main() -> anyhow::Result<()> {
    let cli = crate::cli::Cli::from_env()?;

    setup_tracing(&cli)?;

    cli.run()
}
