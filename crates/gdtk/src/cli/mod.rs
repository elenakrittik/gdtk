use std::path::PathBuf;

use crate::cli::dev::DevCommand;

pub mod context;
#[cfg(any(debug_assertions, feature = "dev"))]
pub mod dev;
pub mod verbosity;

pub struct Cli {
    pub verbosity: crate::cli::verbosity::Verbosity,
    pub command: Option<Command>,
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

pub struct LintCommand {
    /// The GDScript file(s) to lint.
    files: Vec<PathBuf>,
}

pub enum GodotCommand {
    /// List locally-installed or online Godot versions.
    List(GodotListCommand),

    /// Run the specified Godot version.
    Run(GodotRunCommand),

    /// Install the specified Godot version.
    Install(GodotInstallCommand),

    /// Uninstall the specified Godot version.
    Uninstall(GodotUninstallCommand),
}

pub struct GodotListCommand;

pub struct GodotRunCommand {
    /// The Godot version to run.
    version: Option<String>,
}

pub struct GodotInstallCommand {
    /// The Godot version to install.
    version: Option<String>,
}

pub struct GodotUninstallCommand {
    /// The Godot version to uninstall.
    version: Option<String>,
}
