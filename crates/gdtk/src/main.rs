use gdtk::cli::{Commands, GodotCommands};
use gdtk::commands::{godot::run as run_godot, parse::run as run_parse, run as run_main, godot::list::run as run_godot_list};


#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = gdtk::cli::cli();

    match &cli.command {
        Some(Commands::Parse { file }) => run_parse(file)?,
        Some(Commands::Godot { command }) => match command {
            Some(GodotCommands::List { online, unsupported, dev }) => run_godot_list(online, unsupported, dev).await?,
            None => run_godot()?,
        },
        None => run_main()?,
    }

    Ok(())
}
