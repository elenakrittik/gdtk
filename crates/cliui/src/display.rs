use std::fmt::Display;

// TODO: when `-> impl Trait` in `default fn`s gets stable, change the return type to `impl Display`
/// Implement this for your item type to change the apppearance of items of
/// a [crate::Prompt] depending on its state.
///
/// To do that, you must enable the `min_specialization` nightly feature.
/// Otherwise, your specialized impls will conflict with the default impl
/// included with this crate.
pub trait StateDisplay<State> {
    fn display(&self, state: &State) -> String;
}

impl<T: Display, State> StateDisplay<State> for T {
    default fn display(&self, _: &State) -> String {
        self.to_string()
    }
}
