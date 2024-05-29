use std::path::PathBuf;

pub enum DevCommand {
    /// GDScript-related dev commands.
    GDScript(DevGDScriptCommand),
    /// GodotCfg-related dev commands.
    GodotCfg(DevGodotCfgCommand),
}

pub enum DevGDScriptCommand {
    /// Print the result of lexing the specified GDScript file.
    Lex(DevGDScriptLexCommand),
    /// Print the result of parsing the specified GDScript file.
    Parse(DevGDScriptParseCommand),
}

pub struct DevGDScriptLexCommand {
    /// The GDScript file to lex.
    file: PathBuf,
}

pub struct DevGDScriptParseCommand {
/// The GDScript file to parse.
    file: PathBuf,
}

pub enum DevGodotCfgCommand {
    /// Print the result of lexing the specified GodotCfg file.
    Lex(DevGodotCfgLexCommand),
    /// Print the result of parsing the specified GodotCfg file.
    Parse(DevGodotCfgParseCommand),
}

pub struct DevGodotCfgLexCommand 
{
        /// The GodotCfg file to lex.
        file: PathBuf,
    }

pub struct DevGodotCfgParseCommand 
{
        /// The GodotCfg file to parse.
        file: PathBuf,
    }
