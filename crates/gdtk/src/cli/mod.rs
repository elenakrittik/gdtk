use crate::cli::dev::DevCommand;

#[cfg(any(debug_assertions, feature = "dev"))]
pub mod dev;

pub struct Cli {
    pub verbosity: u8,
    pub command: Command,
}

pub enum Command {
    /// Namespace for arbitrary commands useful when working on gdtk.
    #[cfg(any(debug_assertions, feature = "dev"))]
    Dev(DevCommand),
    // /// Manage your Godot installations.
    // Godot(GodotCommand),
    // /// Lint GDScript code.
    // Lint(LintCommand),
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

impl Cli {
    pub fn verbosity(&self) -> tracing::level_filters::LevelFilter {
        todo!()
    }
}

impl tapcli::Command for Cli {
    type Error = anyhow::Error;

    fn parse(parser: &mut tapcli::Parser) -> Result<Self, Self::Error> {
        let mut verbosity = None;

        for arg in parser {
            match arg.as_ref() {
                tapcli::ArgRef::Short('v') => *verbosity.get_or_insert(0u8) += 1,
                tapcli::ArgRef::Long("help") => todo!(),
                tapcli::ArgRef::Value("dev") => todo!(),
                tapcli::ArgRef::Value("godot") => todo!(),
                tapcli::ArgRef::Value("lint") => todo!(),
                other => unknown!(other),
            }
        }

        anyhow::bail!("No command specified.")
    }

    fn run(self) -> Result<Self::Output, Self::Error> {
        todo!()
    }
}
