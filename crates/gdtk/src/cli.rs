use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Parse GDScript source file. Currently mostly broken.
    Parse {
        #[arg(short, long)]
        file: String,
    },
    /// Manage your Godot installations.
    Godot {
        #[command(subcommand)]
        command: Option<GodotCommands>,
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

    /// Install the specified Godot version.
    Install {
        /// The Godot version to install.
        version: String,
<<<<<<< HEAD
    }
=======
    },
>>>>>>> e3c7acc4c6a15018f7d8b2178accdf27a97edf24
}

pub fn cli() -> Cli {
    Cli::parse()
}
