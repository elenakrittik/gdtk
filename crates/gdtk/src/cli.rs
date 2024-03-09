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
    Parse {
        file: PathBuf,
    },
    /// Manage your Godot installations.
    Godot {
        #[command(subcommand)]
        command: GodotCommands,
    },
}

#[derive(Subcommand)]
pub enum GodotCommands {
    /// List locally-installed or online Godot versions.
    List {
        /// List versions avaible online instead of locally-installed ones.
        #[arg(long)]
        online: bool,

        /// Include [unsupported](https://github.com/godotengine/godot-docs/blob/master/about/release_policy.rst)
        /// Godot versions in results. Requires `--online`.
        #[arg(long, requires = "online")]
        unsupported: bool,

        /// Include development snapshots (like dev, alpha, beta, and rc) in results.
        /// Requires `--online`.
        #[arg(long, requires = "online")]
        dev: bool,

        /// Include development snapshots of unsupported versions.
        /// Requires both `--unsupported` and `--dev`.
        #[arg(long = "unsupported-dev", requires = "unsupported", requires = "dev")]
        unsupported_dev: bool,
    },

    /// Run the specified Godot version.
    Run {
        /// The Godot version to run.
        version: String,
    },

    /// Install the specified Godot version.
    Install {
        /// The Godot version to install.
        version: String,
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
