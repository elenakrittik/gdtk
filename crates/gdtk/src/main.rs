use gdtk::cli::{Commands, GodotCommands};
use gdtk::commands::{
<<<<<<< HEAD
    godot::list::run as run_godot_list, godot::run as run_godot, parse::run as run_parse,
    run as run_main, godot::install::run as run_godot_install,
=======
    godot::install::run as run_godot_install, godot::list::run as run_godot_list,
    godot::run as run_godot, parse::run as run_parse, run as run_main,
>>>>>>> e3c7acc4c6a15018f7d8b2178accdf27a97edf24
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
            None => run_godot()?,
        },
        None => run_main()?,
    }

    Ok(())
}
