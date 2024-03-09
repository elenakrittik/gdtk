use gdtk::{
    cli::{Commands, GodotCommands},
    commands as cmds,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = gdtk::cli::cli();

    match cli.command {
        Commands::Parse { file } => cmds::parse::run(file)?,
        Commands::Godot { command } => match command {
            GodotCommands::List {
                online,
                unsupported,
                dev,
                unsupported_dev,
            } => cmds::godot::list::run(online, unsupported, dev, unsupported_dev).await?,
            GodotCommands::Install { version } => cmds::godot::install::run(version).await?,
            GodotCommands::Uninstall { version } => cmds::godot::uninstall::run(version).await?,
            GodotCommands::Run { version } => cmds::godot::run::run(version).await?,
        },
    }

    Ok(())
}
