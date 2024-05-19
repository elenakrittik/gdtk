use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[cfg(any(debug_assertions, feature = "dev"))]
pub mod dev;
pub mod verbosity;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(flatten)]
    pub verbosity: crate::cli::verbosity::Verbosity,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Namespace for arbitrary commands useful when working on gdtk.
    #[cfg(any(debug_assertions, feature = "dev"))]
    Dev {
        #[command(subcommand)]
        command: crate::cli::dev::DevCommands,
    },
    /// Manage your Godot installations.
    Godot {
        #[command(subcommand)]
        command: GodotCommands,
    },
    /// Lint GDScript code.
    Lint {
        /// The GDScript file(s) to lint.
        #[clap(default_value = "./")]
        files: Vec<PathBuf>,
    },
}

#[derive(Subcommand)]
pub enum GodotCommands {
    /// List locally-installed or online Godot versions.
    List,

    /// Run the specified Godot version.
    Run {
        /// The Godot version to run.
        version: Option<String>,
    },

    /// Install the specified Godot version.
    Install {
        /// The Godot version to install.
        version: Option<String>,
    },

    /// Uninstall the specified Godot version.
    Uninstall {
        /// The Godot version to uninstall.
        version: Option<String>,
    },
}

pub fn cli() -> Cli {
    Cli::parse()
}
