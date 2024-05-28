use std::path::PathBuf;

pub enum DevCommand {
    /// GDScript-related dev commands.
    GDScript { command: DevGDScriptCommands },
    /// GodotCfg-related dev commands.
    GodotCfg { command: DevGodotCfgCommands },
}

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

pub enum DevGodotCfgCommands {
    /// Print the result of lexing the specified GodotCfg file.
    Lex {
        /// The GodotCfg file to lex.
        file: PathBuf,
    },
    /// Print the result of parsing the specified GodotCfg file.
    Parse {
        /// The GodotCfg file to parse.
        file: PathBuf,
    },
}
