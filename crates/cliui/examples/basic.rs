use cliui::Prompt;

fn main() -> cliui::Result<()> {
    Prompt::<'_, _, _>::builder()
        .question("Choose Godot version")
        .items(&["4.3", "4.2", "4.1"])
        .build()
        .interact()?;

    Ok(())
}
