use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// [DEV] Parse GDScript source file.
    Parse { file: PathBuf },
    /// Manage your Godot installations.
    Godot {
        #[command(subcommand)]
        command: GodotCommands,
    },
}

#[derive(Subcommand)]
pub enum GodotCommands {
    /// List locally-installed or online Godot versions.
    List,

    /// Run the specified Godot version.
    Run {
        /// The Godot version to run.
        version: String,
    },

    /// Install the specified Godot version.
    Install {
        /// The Godot version to install.
        version: Option<String>,
    },

    /// Uninstall the specified Godot version.
    Uninstall {
        /// The Godot version to uninstall.
        version: String,
    },
}

pub fn cli() -> Cli {
    Cli::parse()
}
