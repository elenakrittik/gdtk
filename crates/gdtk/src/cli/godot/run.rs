use std::process::{Command, Stdio};

use gdtk_gvm::VersionManager;

use crate::cli::{unknown, utils::prompt_local_version};

pub struct GodotRunCommand {
    extra_args: Vec<String>,
}

impl tapcli::Command for GodotRunCommand {
    type Error = anyhow::Error;

    fn parse(parser: &mut tapcli::Parser) -> Result<Self, Self::Error> {
        let mut extra_args = Vec::new();

        for arg in parser {
            match arg {
                tapcli::Arg::Value(value) => extra_args.push(value),
                _ => unknown!(arg),
            }
        }

        Ok(Self { extra_args })
    }

    fn run(self) -> Result<Self::Output, Self::Error> {
        let manager = VersionManager::load()?;
        let version = prompt_local_version(&manager)?;

        let program = version.path().join("godot");

        eprintln!(
            "Running `{}{}`",
            program,
            self.extra_args.iter().fold(String::new(), |mut acc, arg| {
                acc.push(' ');
                acc.push_str(arg.as_str());
                acc
            })
        );

        let mut child = Command::new(program)
            .args(self.extra_args)
            .stdout(Stdio::null())
            .spawn()?;

        child.wait()?;

        Ok(())
    }
}
