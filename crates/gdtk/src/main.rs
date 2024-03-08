use gdtk::cli::{Commands, GodotCommands};
use gdtk::commands::{
    godot::install::run as run_godot_install, godot::list::run as run_godot_list,
    godot::run as run_godot, godot::uninstall::run as run_godot_uninstall, parse::run as run_parse,
    run as run_main,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = gdtk::cli::cli();

    match &cli.command {
        Some(Commands::Parse { file }) => run_parse(file)?,
        Some(Commands::Godot { command }) => match command {
            Some(GodotCommands::List {
                online,
                unsupported,
                dev,
                unsupported_dev,
            }) => run_godot_list(online, unsupported, dev, unsupported_dev).await?,
            Some(GodotCommands::Install { version }) => run_godot_install(version).await?,
            Some(GodotCommands::Uninstall { version }) => run_godot_uninstall(version).await?,
            None => run_godot()?,
        },
        None => run_main()?,
    }

    Ok(())
}
