use std::{
    fmt::{Debug, Display},
    io::Write,
    ops::Range,
};

use console::{Key, Term};
use yansi::Paint;

const QUESTION_MARK_STYLE: yansi::Style = yansi::Style::new().bright_yellow().bold();
const ARROW_STYLE: yansi::Style = yansi::Style::new().bright_green().bold();
const ELLIPSIS_STYLE: yansi::Style = yansi::Style::new().bright_black().bold().dim();
const CHOICE_STYLE: yansi::Style = ARROW_STYLE;
const NO_CHOICE_STYLE: yansi::Style = yansi::Style::new().bright_red().bold();

pub struct DisplaySentinel(());

impl Display for DisplaySentinel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "DisplaySentinel")
    }
}

/// A prompt.
pub struct Prompt<'items, Q: Display, Item: Display> {
    question: Q,
    items: &'items [Item],
    current_item_idx: usize,
    term: Term,
    allow_esc: bool,
    total_view_length: usize,
    view_drag_limit: usize,
    view: SliceView<'items, Item>,
}

impl Prompt<'_, DisplaySentinel, DisplaySentinel> {
    pub fn builder() -> PromptBuilder<'static, DisplaySentinel, DisplaySentinel> {
        PromptBuilder::new()
    }
}

impl<'items, Q: Display, Item: Display> Prompt<'items, Q, Item> {
    pub fn interact(mut self) -> crate::Result<Option<usize>> {
        let mut choice = None;

        self.term.hide_cursor()?;
        self.draw_question()?;

        let mut lines_previously_drawn = self.draw_items()?;

        loop {
            match self.term.read_key()? {
                Key::ArrowUp => {
                    self.move_up()?;
                    self.term.clear_last_lines(lines_previously_drawn)?;

                    lines_previously_drawn = self.draw_items()?;
                }
                Key::ArrowDown => {
                    self.move_down()?;
                    self.term.clear_last_lines(lines_previously_drawn)?;

                    lines_previously_drawn = self.draw_items()?;
                }
                Key::Enter => {
                    choice.replace(self.current_item_idx);
                    break;
                }
                Key::Escape if self.allow_esc => break,
                _ => {}
            }
        }

        self.term.clear_last_lines(lines_previously_drawn)?;
        self.term.clear_last_lines(1)?;
        self.draw_choice(choice)?;
        self.term.show_cursor()?;

        Ok(choice)
    }

    fn draw_question(&mut self) -> crate::Result<usize> {
        writeln!(
            self.term,
            "{} {}",
            '?'.paint(QUESTION_MARK_STYLE),
            self.question
        )?;

        Ok(1)
    }

    fn draw_items(&mut self) -> crate::Result<usize> {
        let range = self.view.start()..=self.view.end();
        let has_items_above = self.view.start() > 0;
        let has_items_below = self.view.slice.len().saturating_sub(self.view.end()) > 1;
        let mut lines_drawn = 0;

        if has_items_above {
            writeln!(self.term, "{}", "...".paint(ELLIPSIS_STYLE))?;
            lines_drawn += 1;
        }

        let items = self
            .view
            .slice
            .iter()
            .enumerate()
            .filter(|(idx, _)| range.contains(idx));

        for (idx, item) in items {
            let arrow = if idx == self.current_item_idx {
                ">"
            } else {
                " "
            };

            writeln!(self.term, "{} {}", arrow.paint(ARROW_STYLE), item)?;

            lines_drawn += 1;
        }

        if has_items_below {
            writeln!(self.term, "{}", "...".paint(ELLIPSIS_STYLE))?;
            lines_drawn += 1;
        }

        Ok(lines_drawn)
    }

    fn draw_choice(&mut self, choice: Option<usize>) -> crate::Result {
        if let Some(choice) = choice {
            writeln!(
                self.term,
                "{} {}: {}",
                '?'.paint(CHOICE_STYLE),
                self.question,
                &self.items[choice],
            )?;
        } else {
            writeln!(
                self.term,
                "{} {}",
                'x'.paint(NO_CHOICE_STYLE),
                self.question,
            )?;
        }

        Ok(())
    }

    fn move_up(&mut self) -> crate::Result {
        self.current_item_idx = self.current_item_idx.saturating_sub(1);

        if self.view.end().saturating_sub(self.current_item_idx) > self.view_drag_limit {
            self.view.shift(-1);
        }

        Ok(())
    }

    fn move_down(&mut self) -> crate::Result {
        let next_item_idx = self.current_item_idx.saturating_add(1);

        if next_item_idx < self.view.slice.len() {
            self.current_item_idx = next_item_idx;
        }

        if self.current_item_idx.saturating_sub(self.view.start()) > self.view_drag_limit {
            self.view.shift(1);
        }

        Ok(())
    }
}

impl<Q, Item> Debug for Prompt<'_, Q, Item>
where
    Q: Display + Debug,
    Item: Display + Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Prompt")
            .field("question", &self.question)
            .field("items", &self.items)
            .field("current_item_idx", &self.current_item_idx)
            .field("term", &self.term)
            .field("allow_esc", &self.allow_esc)
            .field("total_view_length", &self.total_view_length)
            .field("view_drag_limit", &self.view_drag_limit)
            .field("view", &self.view)
            .finish()
    }
}

struct SliceView<'a, T> {
    slice: &'a [T],
    range: Range<usize>,
    length: usize,
}

impl<'a, T> SliceView<'a, T> {
    fn new(slice: &'a [T], range: Range<usize>, length: usize) -> Self {
        Self {
            slice,
            range,
            length,
        }
    }

    fn start(&self) -> usize {
        self.range.start
    }

    fn end(&self) -> usize {
        self.range.end
    }

    fn shift(&mut self, delta: isize) {
        let start = self.range.start.saturating_add_signed(delta);
        let end = self.range.end.saturating_add_signed(delta);

        if end < self.slice.len() && end.saturating_sub(start) == self.length {
            self.range.start = start;
            self.range.end = end;
        }
    }
}

impl<'a, T> Debug for SliceView<'a, T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SliceView")
            .field("slice", &self.slice)
            .field("range", &self.range)
            .field("length", &self.length)
            .finish()
    }
}

pub struct PromptBuilder<'items, Q: Display, Item: Display> {
    question: Option<Q>,
    items: Option<&'items [Item]>,
    default_item_idx: Option<usize>,
    term: Option<Term>,
    allow_esc: Option<bool>,
    view_length: Option<usize>,
    view_drag_limit: Option<usize>,
}

impl PromptBuilder<'_, DisplaySentinel, DisplaySentinel> {
    fn new() -> Self {
        Self {
            question: None,
            items: None,
            default_item_idx: None,
            term: None,
            allow_esc: None,
            view_length: None,
            view_drag_limit: None,
        }
    }
}

impl<'items, Q: Display, Item: Display> PromptBuilder<'items, Q, Item> {
    pub fn with_question<NewQ: Display>(self, question: NewQ) -> PromptBuilder<'items, NewQ, Item> {
        PromptBuilder {
            question: Some(question),
            ..self
        }
    }

    pub fn with_items<NewItem: Display>(self, items: &[NewItem]) -> PromptBuilder<'_, Q, NewItem> {
        PromptBuilder {
            items: Some(items),
            ..self
        }
    }

    pub fn with_default_item(mut self, idx: usize) -> Self {
        self.default_item_idx = Some(idx);
        self
    }

    pub fn with_term(mut self, term: Term) -> Self {
        self.term = Some(term);
        self
    }

    pub fn allow_esc(mut self, allow: bool) -> Self {
        self.allow_esc = Some(allow);
        self
    }

    pub fn with_view_length(mut self, length: usize) -> Self {
        self.view_length = Some(length);
        self
    }

    pub fn with_view_drag_limit(mut self, limit: usize) -> Self {
        self.view_drag_limit = Some(limit);
        self
    }

    #[rustfmt::skip]
    pub fn build(self) -> Prompt<'items, Q, Item> {
        let question = self.question.expect("`question` must've been set before calling `.build()`");
        let items = self.items.expect("`items` must've been set before calling `.build()`");
        let current_item_idx = self.default_item_idx.unwrap_or_default();
        let term = self.term.unwrap_or_else(Term::stderr);
        let allow_esc = self.allow_esc.unwrap_or(false);
        let total_view_length = self.view_length.unwrap_or(7);
        let view_drag_limit = self.view_drag_limit.unwrap_or(3);
        let view = SliceView::new(
            items,
            current_item_idx..total_view_length,
            total_view_length,
        );

        Prompt {
            question,
            items,
            current_item_idx,
            term,
            allow_esc,
            total_view_length,
            view_drag_limit,
            view,
        }
    }
}
