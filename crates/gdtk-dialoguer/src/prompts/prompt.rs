use std::{
    fmt::{Debug, Display},
    io::Write,
    ops::Range,
};

use console::{Key, Term};
use typed_builder::TypedBuilder;
use yansi::Paint;

const QUESTION_MARK_STYLE: yansi::Style = yansi::Style::new().bright_yellow().bold();
const ARROW_STYLE: yansi::Style = yansi::Style::new().bright_green().bold();
const ELLIPSIS_STYLE: yansi::Style = yansi::Style::new().bright_black().bold().dim();
const CHOICE_STYLE: yansi::Style = ARROW_STYLE;
const NO_CHOICE_STYLE: yansi::Style = yansi::Style::new().bright_red().bold();

#[derive(TypedBuilder)]
pub struct Prompt<
    'items,
    Q: Display,
    Item: Display,
    const TOTAL_VIEW_LENGTH: usize = 7,
    const VIEW_DRAG_LIMIT: usize = 3,
> {
    question: Q,
    items: &'items [Item],
    #[builder(default)]
    current_item_idx: usize,
    #[builder(default = Term::stderr())]
    term: Term,
}

impl<'items, Q, Item, const TOTAL_VIEW_LENGTH: usize, const VIEW_DRAG_LIMIT: usize>
    Prompt<'items, Q, Item, TOTAL_VIEW_LENGTH, VIEW_DRAG_LIMIT>
where
    Q: Display,
    Item: Display,
{
    pub fn interact(mut self) -> crate::Result<Option<&'items Item>> {
        let mut choice = None;
        let mut view = SliceView::new(self.items, self.current_item_idx, TOTAL_VIEW_LENGTH);

        self.term.hide_cursor()?;
        self.draw_question()?;

        let mut lines_previously_drawn = self.draw_items(&mut view)?;

        loop {
            match self.term.read_key()? {
                Key::ArrowUp => {
                    self.move_up(&mut view)?;
                    self.term.clear_last_lines(lines_previously_drawn)?;

                    lines_previously_drawn = self.draw_items(&mut view)?;
                }
                Key::ArrowDown => {
                    self.move_down(&mut view)?;
                    self.term.clear_last_lines(lines_previously_drawn)?;

                    lines_previously_drawn = self.draw_items(&mut view)?;
                }
                Key::Enter => {
                    choice.replace(&self.items[self.current_item_idx]);
                    break;
                }
                Key::Escape => break,
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

    fn draw_items(
        &mut self,
        view: &mut SliceView<'items, Item, TOTAL_VIEW_LENGTH>,
    ) -> crate::Result<usize> {
        let range = view.range.start..=view.range.end;
        let has_items_above = view.start() > 0;
        let has_items_below = view.slice.len().saturating_sub(view.end()) > 1;
        let mut lines_drawn = 0;

        if has_items_above {
            writeln!(self.term, "{}", "...".paint(ELLIPSIS_STYLE))?;
            lines_drawn += 1;
        }

        let items = view
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

    fn draw_choice(&mut self, choice: Option<&'items Item>) -> crate::Result {
        if let Some(choice) = choice {
            writeln!(
                self.term,
                "{} {}: {}",
                '?'.paint(CHOICE_STYLE),
                self.question,
                choice
            )?;
        } else {
            writeln!(
                self.term,
                "{} {}",
                'x'.paint(NO_CHOICE_STYLE),
                self.question
            )?;
        }

        Ok(())
    }

    fn move_up(&mut self, view: &mut SliceView<'items, Item, TOTAL_VIEW_LENGTH>) -> crate::Result {
        self.current_item_idx = self.current_item_idx.saturating_sub(1);

        if view.end().saturating_sub(self.current_item_idx) > VIEW_DRAG_LIMIT {
            view.shift(-1);
        }

        Ok(())
    }

    fn move_down(
        &mut self,
        view: &mut SliceView<'items, Item, TOTAL_VIEW_LENGTH>,
    ) -> crate::Result {
        let next_item_idx = self.current_item_idx.saturating_add(1);

        if next_item_idx < view.slice.len() {
            self.current_item_idx = next_item_idx;
        }

        if self.current_item_idx.saturating_sub(view.start()) > VIEW_DRAG_LIMIT {
            view.shift(1);
        }

        Ok(())
    }
}

struct SliceView<'a, T, const LENGTH: usize> {
    slice: &'a [T],
    range: Range<usize>,
}

impl<'a, T, const LENGTH: usize> SliceView<'a, T, LENGTH> {
    fn new(slice: &'a [T], from: usize, to: usize) -> Self {
        Self {
            slice,
            range: from..to,
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

        #[rustfmt::skip]
        if end < self.slice.len() && end.saturating_sub(start) == LENGTH {
            self.range.start = start;
            self.range.end = end;
        }
    }
}

impl<'a, T, const LENGTH: usize> Debug for SliceView<'a, T, LENGTH>
where
    T: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SliceView")
            .field("slice", &self.slice)
            .field("range", &self.range)
            .finish()
    }
}
