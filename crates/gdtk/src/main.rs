#[cfg(any(debug_assertions, feature = "dev"))]
use gdtk::cli::DevCommands;
use gdtk::{
    cli::{Commands, GodotCommands},
    commands as cmds,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = gdtk::cli::cli();

    match cli.command {
        #[cfg(any(debug_assertions, feature = "dev"))]
        Commands::Dev { command } => match command {
            DevCommands::Lex { file } => cmds::dev::lex::run(file)?,
            DevCommands::Parse { file } => cmds::dev::parse::run(file)?,
            DevCommands::GodotCfg { file } => cmds::dev::godotcfg::run(file)?,
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
