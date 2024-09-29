use std::fmt::Display;

pub trait StateDisplay<State> {
    fn display(&self, state: &State) -> impl Display;
}

// TODO: wait until `-> impl Trait` becomes stable in default impls and drop the feature-gated Display impl
impl<T: Display, State> StateDisplay<State> for T {
    fn display(&self, _: &State) -> impl Display {
        self
    }
}
