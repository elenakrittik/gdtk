use cliui::{Action, Key, Prompt};

struct MyState {
    only_3x: bool,
}

fn main() -> cliui::Result<()> {
    const ITEMS: [&str; 7] = ["4.3", "4.2", "4.1", "4.0", "3.6", "3.5", "3.4"];

    Prompt::builder()
        .with_question("Choose Godot version")
        .with_items(ITEMS.into_iter().collect::<Vec<_>>())
        .with_state(MyState { only_3x: false })
        .with_action(
            Key::Char('b'),
            Action {
                description: "Toggle something idk",
                callback: |prompt| {
                    prompt.state_mut().only_3x = !prompt.state().only_3x;

                    // TODO: implement filtering

                    Ok(())
                },
            },
        )
        .build()
        .interact()?;

    Ok(())
}
