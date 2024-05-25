use std::path::PathBuf;

use clap::Subcommand;

#[derive(Subcommand)]
pub enum DevCommands {
    /// GDScript-related dev commands.
    #[clap(name = "gdscript")]
    GDScript {
        #[command(subcommand)]
        command: DevGDScriptCommands,
    },
    /// GodotCfg-related dev commands.
    #[clap(name = "godotcfg")]
    GodotCfg {
        #[command(subcommand)]
        command: DevGodotCfgCommands,
    },
}

#[derive(Subcommand)]
pub enum DevGDScriptCommands {
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
pub enum DevGodotCfgCommands {
    /// Print the result of lexing the specified GodotCfg file.
    Lex {
        /// The GodotCfg file to lex.
        #[clap(default_value = "project.godot")]
        file: PathBuf,
    },
    /// Print the result of parsing the specified GodotCfg file.
    Parse {
        /// The GodotCfg file to parse.
        #[clap(default_value = "project.godot")]
        file: PathBuf,
    },
}
