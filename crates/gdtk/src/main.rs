#[cfg(any(debug_assertions, feature = "dev"))]
use gdtk::cli::dev::{DevCommand, DevGDScriptCommands, DevGodotCfgCommands};
use gdtk::{
    cli::{Command, GodotCommand},
    commands as cmds,
    utils::setup_tracing,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = gdtk::cli::cli()?;

    setup_tracing(&cli)?;

    match cli.command {
        #[cfg(any(debug_assertions, feature = "dev"))]
        Command::Dev { command } => match command {
            DevCommand::GDScript { command } => match command {
                DevGDScriptCommands::Lex { file } => cmds::dev::gdscript::lex::run(file)?,
                DevGDScriptCommands::Parse { file } => cmds::dev::gdscript::parse::run(file)?,
            },
            DevCommand::GodotCfg { command } => match command {
                DevGodotCfgCommands::Lex { file } => cmds::dev::godotcfg::lex::run(file)?,
                DevGodotCfgCommands::Parse { file } => cmds::dev::godotcfg::parse::run(file)?,
            },
        },
        Command::Godot { command } => match command {
            GodotCommand::List => cmds::godot::list::run()?,
            GodotCommand::Install { version } => cmds::godot::install::run(version).await?,
            GodotCommand::Uninstall { version } => cmds::godot::uninstall::run(version).await?,
            GodotCommand::Run { version } => cmds::godot::run::run(version).await?,
        },
        Command::Lint { files } => cmds::lint::run(files)?,
    }

    Ok(())
}
