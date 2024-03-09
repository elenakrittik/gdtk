use gdtk::cli::{Commands, GodotCommands};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = gdtk::cli::cli();

    match cli.command {
        Commands::Parse { file } => gdtk::commands::parse::run(file)?,
        Commands::Godot { command } => match command {
            GodotCommands::List {
                online,
                unsupported,
                dev,
                unsupported_dev,
            } => run_godot_list(online, unsupported, dev, unsupported_dev).await?,
            GodotCommands::Install { version } => run_godot_install(version).await?,
            GodotCommands::Uninstall { version } => run_godot_uninstall(version).await?,
            GodotCommands::Run { version } => run_godot_run(version).await?,
            None => run_godot()?,
        },
    }

    Ok(())
}
