use cliui::Prompt;

fn main() -> cliui::Result<()> {
    const ITEMS: [&str; 3] = ["4.3", "4.2", "4.1"];

    Prompt::builder()
        .with_question("Choose Godot version")
        .with_items(&ITEMS)
        .build()
        .interact()?;

    Ok(())
}
