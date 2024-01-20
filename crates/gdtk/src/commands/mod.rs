use owo_colors::OwoColorize;

use crate::display::{print_error, ACTION};

<<<<<<< HEAD
pub mod parse;
pub mod godot;

pub fn run() -> anyhow::Result<()> {
    print_error(format!("No command specififed. Run {} to view available options.", "`gdtk help`".style(*ACTION)));
=======
pub mod godot;
pub mod parse;

pub fn run() -> anyhow::Result<()> {
    print_error(format!(
        "No command specififed. Run {} to view available options.",
        "`gdtk help`".style(*ACTION)
    ));
>>>>>>> e3c7acc4c6a15018f7d8b2178accdf27a97edf24
    Ok(())
}
