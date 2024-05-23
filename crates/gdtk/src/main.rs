#[cfg(any(debug_assertions, feature = "dev"))]
use gdtk::cli::dev::{DevCommands, DevGDScriptCommands, DevGodotCfgCommands};
use gdtk::{
    cli::{Commands, GodotCommands},
    commands as cmds,
    utils::setup_tracing,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = gdtk::cli::cli();

    setup_tracing(&cli)?;

    match cli.command {
        #[cfg(any(debug_assertions, feature = "dev"))]
        Commands::Dev { command } => match command {
            DevCommands::GDScript { command } => match command {
                DevGDScriptCommands::Lex { file } => cmds::dev::gdscript::lex::run(file)?,
                DevGDScriptCommands::Parse { file } => cmds::dev::gdscript::parse::run(file)?,
                DevGDScriptCommands::Render => cmds::dev::gdscript::render::run()?,
            },
            DevCommands::GodotCfg { command } => match command {
                DevGodotCfgCommands::Lex { file } => cmds::dev::godotcfg::lex::run(file)?,
                DevGodotCfgCommands::Parse { file } => cmds::dev::godotcfg::parse::run(file)?,
            },
        },
        Commands::Godot { command } => match command {
            GodotCommands::List => cmds::godot::list::run()?,
            GodotCommands::Install { version } => cmds::godot::install::run(version).await?,
            GodotCommands::Uninstall { version } => cmds::godot::uninstall::run(version).await?,
            GodotCommands::Run { version } => cmds::godot::run::run(version).await?,
        },
        Commands::Lint { files } => cmds::lint::run(files)?,
    }

    Ok(())
}
