use tracing::level_filters::LevelFilter;

use crate::cli::{dev::DevCommand, godot::GodotCommand, lint::LintCommand};

#[cfg(any(debug_assertions, feature = "dev"))]
pub mod dev;
pub mod godot;
pub mod lint;
pub mod utils;

pub struct Cli {
    pub verbosity: u8,
    pub command: Command,
}

impl Cli {
    pub fn verbosity(&self) -> LevelFilter {
        match self.verbosity {
            0 => LevelFilter::INFO,
            1 => LevelFilter::ERROR,
            2 => LevelFilter::WARN,
            3 => LevelFilter::INFO,
            4 => LevelFilter::DEBUG,
            5 => LevelFilter::TRACE,
            _ => unreachable!(),
        }
    }
}

impl tapcli::Command for Cli {
    type Error = anyhow::Error;

    async fn parse(parser: &mut tapcli::Parser) -> Result<Self, Self::Error> {
        let mut verbosity = None;

        while let Some(arg) = parser.peek() {
            match arg.as_ref() {
                tapcli::ArgRef::Short('v') => {
                    *verbosity.get_or_insert(0u8) += 1;
                    parser.next();
                }
                tapcli::ArgRef::Long("help") => todo!(),
                tapcli::ArgRef::Value("dev" | "godot" | "lint") => {
                    return Ok(Self {
                        verbosity: verbosity.unwrap_or(0),
                        command: Command::parse(parser).await?,
                    });
                }
                other => unknown!(other),
            }
        }

        anyhow::bail!("No command specified.")
    }

    async fn run(self) -> Result<Self::Output, Self::Error> {
        self.command.run().await
    }
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

impl tapcli::Command for Command {
    type Error = anyhow::Error;

    async fn parse(parser: &mut tapcli::Parser) -> Result<Self, Self::Error> {
        let command = match parser.next().unwrap().as_ref() {
            #[cfg(any(debug_assertions, feature = "dev"))]
            tapcli::ArgRef::Value("dev") => Self::Dev(DevCommand::parse(parser).await?),
            tapcli::ArgRef::Value("godot") => Self::Godot(GodotCommand::parse(parser).await?),
            tapcli::ArgRef::Value("lint") => Self::Lint(LintCommand::parse(parser).await?),
            _ => unreachable!(),
        };

        if let Some(arg) = parser.next() {
            anyhow::bail!("Unrecognized argument: {arg}");
        }

        Ok(command)
    }

    async fn run(self) -> Result<Self::Output, Self::Error> {
        match self {
            #[cfg(any(debug_assertions, feature = "dev"))]
            Self::Dev(cmd) => cmd.run().await,
            Self::Godot(cmd) => cmd.run().await,
            Self::Lint(cmd) => cmd.run().await,
        }
    }
}

pub macro unknown($arg:expr) {
    ::anyhow::bail!("Unknown option: {:?}", $arg)
}

pub macro missing($what:expr) {
    ::anyhow::bail!(concat!("Missing required argument: ", $what))
}
