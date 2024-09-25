use crate::Prompt;

type ActionCallback<Item, State> = fn(&mut Prompt<Item, State>) -> crate::Result;

/// An action of a [Prompt].
pub struct Action<Item, State> {
    pub description: &'static str,
    pub callback: ActionCallback<Item, State>,
}
