use std::env::consts::EXE_SUFFIX;

use crate::cli::{
    godot::{
        default::GodotDefaultCommand, install::GodotInstallCommand, list::GodotListCommand,
        run::GodotRunCommand, uninstall::GodotUninstallCommand,
    },
    unknown,
};

pub mod default;
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

    /// Change the default Godot version.
    Default(GodotDefaultCommand),
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
            tapcli::ArgRef::Value("default") => Ok(Self::Default(GodotDefaultCommand::parse(parser)?)),
            other => unknown!(other),
        }
    }

    fn run(self) -> Result<Self::Output, Self::Error> {
        match self {
            GodotCommand::List(c) => c.run(),
            GodotCommand::Run(c) => c.run(),
            GodotCommand::Install(c) => c.run(),
            GodotCommand::Uninstall(c) => c.run(),
            GodotCommand::Default(c) => c.run(),
        }
    }
}

fn symlink_default_version(
    installation_folder: &gdtk_paths::camino::Utf8Path,
) -> anyhow::Result<()> {
    let original = installation_folder.join(format!("godot{}", EXE_SUFFIX));
    let link = gdtk_paths::default_godot_path()?;

    if link.exists() {
        std::fs::remove_file(&link)?;
    }

    #[cfg(windows)]
    {
        // std::os::windows::fs::symlink_file(original, link)?;
        mslnk::ShellLink::new(original)?.create_lnk(link)?;
    }

    #[cfg(not(windows))]
    {
        std::os::unix::fs::symlink(original, link)?;
    }

    Ok(())
}
