use ahash::AHashMap;
use console::{Key, Term};

use super::{action::Action, sentinel::Sentinel, vecview::VecView};
use crate::{Prompt, StateDisplay};

impl Prompt<Sentinel, Sentinel> {
    pub fn builder() -> PromptBuilder<Sentinel> {
        PromptBuilder::new()
    }
}

pub struct PromptBuilder<Item, State = ()> {
    question: Option<&'static str>,
    items: Option<Vec<Item>>,
    // default_item_idx: Option<usize>,
    term: Option<Term>,
    allow_esc: Option<bool>,
    view_length: Option<usize>,
    view_drag_limit: Option<usize>,
    actions: AHashMap<Key, Action<Item, State>>,
    state: State,
}

impl PromptBuilder<Sentinel> {
    fn new() -> Self {
        Self {
            question: None,
            items: None,
            // default_item_idx: None,
            term: None,
            allow_esc: None,
            view_length: None,
            view_drag_limit: None,
            actions: AHashMap::new(),
            state: (),
        }
    }
}

impl<State, Item: StateDisplay<State>> PromptBuilder<Item, State> {
    /// Set the question for this prompt.
    pub fn with_question(mut self, question: &'static str) -> PromptBuilder<Item, State> {
        self.question = Some(question);
        self
    }

    /// Set the items for this prompt. **This resets `.actions`.**.
    pub fn with_items<NewItem: StateDisplay<State>>(
        self,
        items: impl Into<Vec<NewItem>>,
    ) -> PromptBuilder<NewItem, State> {
        PromptBuilder {
            items: Some(items.into()),
            actions: AHashMap::new(),
            ..self
        }
    }

    /// Set the default state for this prompt. **This resets `.actions`.**.
    pub fn with_state<NewState>(self, state: NewState) -> PromptBuilder<Item, NewState> {
        PromptBuilder {
            state,
            actions: AHashMap::new(),
            ..self
        }
    }

    /// Set the default item for this prompt.
    ///
    /// Defaults to `0`.
    // pub fn with_default_item(mut self, idx: usize) -> Self {
    //     self.default_item_idx = Some(idx);
    //     self
    // }

    /// Set the terminal for this prompt.
    ///
    /// Defaults to [Term::stderr].
    pub fn with_term(mut self, term: Term) -> Self {
        self.term = Some(term);
        self
    }

    /// Set whether to allow exiting the prompt with ESC for this prompt.
    ///
    /// Defaults to `true`.
    pub fn allow_esc(mut self, allow: bool) -> Self {
        self.allow_esc = Some(allow);
        self
    }

    /// Set the view length for this prompt.
    ///
    /// Defaults to `7`.
    pub fn with_view_length(mut self, length: usize) -> Self {
        self.view_length = Some(length);
        self
    }

    /// Set the view drag limit for this prompt. To maintain prompt's cursor
    /// in the middle, this should be set to `floor(view_length / 2)`.
    ///
    /// Defaults to `3`.
    pub fn with_view_drag_limit(mut self, limit: usize) -> Self {
        self.view_drag_limit = Some(limit);
        self
    }

    /// Bind a key to an action for this prompt.
    pub fn with_action(mut self, key: Key, action: Action<Item, State>) -> Self {
        self.actions.insert(key, action);
        self
    }

    #[rustfmt::skip]
    pub fn build(self) -> Prompt<Item, State> {
        let question = self.question.expect("`question` must've been set before calling `.build()`");
        let items = self.items.expect("`items` must've been set before calling `.build()`");
        assert!(!items.is_empty());
        let term = self.term.unwrap_or_else(Term::stderr);
        let allow_esc = self.allow_esc.unwrap_or(false);
        let total_view_length = self.view_length.unwrap_or(7);
        let view_drag_limit = self.view_drag_limit.unwrap_or(3);
        let view = VecView::new(
            items,
            total_view_length,
            view_drag_limit,
        );
        let actions = self.actions;
        let state = self.state;

        Prompt {
            question,
            term,
            allow_esc,
            view,
            actions,
            state,
        }
    }
}
