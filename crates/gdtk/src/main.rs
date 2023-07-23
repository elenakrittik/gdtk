use gdtk::commands::parse::run as parse;
use gdtk::display::{print_error, ACTION};
use owo_colors::OwoColorize;

fn main() -> anyhow::Result<()> {
    let cli = gdtk::cli::parse();

    match &cli.command {
        Some(gdtk::cli::Commands::Parse { file }) => match parse(file) {
            Ok(_) => (),
            Err(e) => print_error(e.to_string()),
        },
        None => {
            print_error(format!(
                "no command specified. Run {} to view help message.",
                "`gdtk help`".style(*ACTION)
            ));
        }
    }

    Ok(())
}
