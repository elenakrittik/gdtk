#![deny(clippy::disallowed_types)]
#![feature(
    let_chains,
    decl_macro,
    option_get_or_insert_default,
    impl_trait_in_assoc_type
)]
#![feature(generic_arg_infer)]

use tapcli::Command;

use crate::utils::setup_tracing;

pub mod cli;
pub mod utils;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = crate::cli::Cli::from_env().await?;

    setup_tracing(&cli)?;

    cli.run().await
}
