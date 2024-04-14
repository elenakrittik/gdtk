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
    /// Namespace for arbitrary commands useful when working on gdtk.
    #[cfg(any(debug_assertions, feature = "dev"))]
    Dev {
        #[command(subcommand)]
        command: DevCommands,
    },
    /// Manage your Godot installations.
    Godot {
        #[command(subcommand)]
        command: GodotCommands,
    },
}

#[derive(Subcommand)]
pub enum DevCommands {
    /// Print the result of lexing the specified GDScript file.
    Lex {
        /// The GDScript file to lex.
        file: PathBuf,
    },
    /// Print the result of parsing the specified GDScript file.
    Parse {
        /// The GDScript file to parse.
        file: PathBuf,
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
