#![deny(clippy::disallowed_types)]
#![feature(
    let_chains,
    decl_macro,
    option_get_or_insert_default,
    impl_trait_in_assoc_type
)]

use tapcli::Command;

use crate::utils::setup_tracing;

pub mod cli;
pub mod utils;

// #[cfg(any(debug_assertions, feature = "dev"))]
// use gdtk::cli::dev::{DevCommand, DevGDScriptCommands, DevGodotCfgCommands};
// use gdtk::{
//     cli::{Command, GodotCommand},
//     commands as cmds,
//     utils::setup_tracing,
// };

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = crate::cli::Cli::from_env().await?;

    setup_tracing(&cli)?;

    cli.run().await
}
