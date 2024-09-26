use crate::cli::{
    godot::{
        install::GodotInstallCommand, list::GodotListCommand, run::GodotRunCommand,
        uninstall::GodotUninstallCommand,
    },
    unknown,
};

pub mod install;
pub mod list;
pub mod run;
pub mod uninstall;

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

impl tapcli::Command for GodotCommand {
    type Error = anyhow::Error;

    #[rustfmt::skip]
    fn parse(parser: &mut tapcli::Parser) -> Result<Self, Self::Error> {
        match parser.next().unwrap().as_ref() {
            tapcli::ArgRef::Value("list") => Ok(Self::List(GodotListCommand::parse(parser)?)),
            tapcli::ArgRef::Value("run") => Ok(Self::Run(GodotRunCommand::parse(parser)?)),
            tapcli::ArgRef::Value("install") => Ok(Self::Install(GodotInstallCommand::parse(parser)?)),
            tapcli::ArgRef::Value("uninstall") => Ok(Self::Uninstall(GodotUninstallCommand::parse(parser)?)),
            other => unknown!(other),
        }
    }

    fn run(self) -> Result<Self::Output, Self::Error> {
        match self {
            GodotCommand::List(c) => c.run(),
            GodotCommand::Run(c) => c.run(),
            GodotCommand::Install(c) => c.run(),
            GodotCommand::Uninstall(c) => c.run(),
        }
    }
}

// fn symlink_default_version(version_folder: &std::path::Path) -> std::io::Result<()> {
//     let original = version_folder.join("godot");
//     let link = gdtk_paths::default_godot_path()?;

//     if link.exists() {
//         std::fs::remove_file(&link)?;
//     }

//     #[cfg(windows)]
//     {
//         std::os::windows::fs::symlink_file(original, link)?;
//     }

//     #[cfg(not(windows))]
//     {
//         std::os::unix::fs::symlink(original, link)?;
//     }

//     Ok(())
// }
