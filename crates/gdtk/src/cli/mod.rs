use std::path::PathBuf;

use crate::cli::{dev::DevCommand, verbosity::Verbosity};

pub mod context;
#[cfg(any(debug_assertions, feature = "dev"))]
pub mod dev;
pub mod verbosity;

pub struct Cli {
    pub verbosity: Verbosity,
    pub command: Command,
}

pub enum Command {
    /// Namespace for arbitrary commands useful when working on gdtk.
    #[cfg(any(debug_assertions, feature = "dev"))]
    Dev(DevCommand),
    /// Manage your Godot installations.
    Godot(GodotCommand),
    /// Lint GDScript code.
    Lint(LintCommand),
}

// pub struct LintCommand {
//     /// The GDScript file(s) to lint.
//     files: Vec<PathBuf>,
// }

// pub enum GodotCommand {
//     /// List locally-installed or online Godot versions.
//     List(GodotListCommand),

//     /// Run the specified Godot version.
//     Run(GodotRunCommand),

//     /// Install the specified Godot version.
//     Install(GodotInstallCommand),

//     /// Uninstall the specified Godot version.
//     Uninstall(GodotUninstallCommand),
// }

// pub struct GodotListCommand;

// pub struct GodotRunCommand {
//     /// The Godot version to run.
//     version: String,
// }

// pub struct GodotInstallCommand {
//     /// The Godot version to install.
//     version: String,
// }

// pub struct GodotUninstallCommand {
//     /// The Godot version to uninstall.
//     version: String,
// }

pub macro unknown($arg:expr) {
    ::anyhow::bail!("Unknown option: {:?}", $arg)
}

impl tapcli::Command for Cli {
    pub fn parse(parser: &mut tapcli::Parser) -> Result<Self, Self::Error> {
        let mut verbosity = Option<0u8>;

        while let Some(arg) = parser.next()? {
            match arg {
                lexopt::Arg::Short('v') => verbosity.get_or_insert_default() += 1,
                lexopt::Arg::Long("help") => todo!(),
                lexopt::Arg::Value("dev") => todo!(),
                lexopt::Arg::Value("godot") => todo!(),
                lexopt::Arg::Value("lint") => todo!(),
                other => unknown!(other),
            }
        }
    }

    pub fn run(self) -> Result<Self::Output, Self::Error> {
        todo!()
    }
}
