use std::{io::Write, ops::ControlFlow};

use console::{Key, Term};
use typed_builder::TypedBuilder;
use yansi::Paint;

const LIMIT: usize = 3;
const QUESTION_STYLE: yansi::Style = yansi::Style::new().yellow().bold();
const ARROW_STYLE: yansi::Style = yansi::Style::new().bright_green().bold();
const ELLIPSIS_STYLE: yansi::Style = yansi::Style::new().white().dim();

pub type Question<'a> = &'a str;
pub type Items<'a> = Vec<&'a str>;
pub type Step<'a> = (Question<'a>, Items<'a>);
pub type QueryResult<'a> = ControlFlow<(), Step<'a>>;
pub type QueryFn<'a> = fn(&DynamicPrompt) -> QueryResult<'a>;

#[derive(TypedBuilder)]
pub struct DynamicPrompt<'a> {
    query: QueryFn<'a>,
    #[builder(default)]
    current_choice: Option<&'a str>,
    #[builder(default = Term::stderr())]
    term: Term,
}

impl<'a> DynamicPrompt<'a> {
    pub fn interact(mut self) -> crate::Result<Option<&'a str>> {
        self.term.hide_cursor()?;

        while let ControlFlow::Continue(step) = (self.query)(&self) {
            self.process_step(step)?;
        }

        self.term.show_cursor()?;

        Ok(self.current_choice)
    }

    fn process_step(&mut self, step: Step<'a>) -> crate::Result {
        let mut current_item_idx = 0;
        let lines_drawn_previously = self.draw_prompt(current_item_idx, &step)?;

        loop {
            match self.term.read_key()? {
                Key::ArrowDown => {
                    if current_item_idx < step.1.len() - 1 {
                        current_item_idx += 1;
                        self.redraw_prompt(current_item_idx, &step, lines_drawn_previously)?;
                    }
                }
                Key::ArrowUp => {
                    if current_item_idx > 0 {
                        current_item_idx -= 1;
                        self.redraw_prompt(current_item_idx, &step, lines_drawn_previously)?;
                    }
                }
                Key::Enter => break,
                _ => {}
            }
        }

        self.current_choice = Some(step.1[current_item_idx]);

        Ok(())
    }

    fn redraw_prompt(
        &mut self,
        current_item_idx: usize,
        step: &Step<'a>,
        lines_drawn_previously: usize,
    ) -> crate::Result<usize> {
        self.term.clear_last_lines(lines_drawn_previously)?;

        self.draw_prompt(current_item_idx, step)
    }

    fn draw_prompt(&mut self, current_item_idx: usize, step: &Step<'a>) -> crate::Result<usize> {
        let mut lines_drawn: usize = 0;

        lines_drawn += self.draw_question(step.0)?;
        lines_drawn += self.draw_items(current_item_idx, step.1.as_slice())?;

        self.term.flush()?;

        Ok(lines_drawn)
    }

    fn draw_question(&mut self, question: Question<'a>) -> crate::Result<usize> {
        writeln!(self.term, "{} {}", '?'.paint(QUESTION_STYLE), question)?;

        Ok(1)
    }

    fn draw_items(
        &mut self,
        current_item_idx: usize,
        items: &<Items<'a> as std::ops::Deref>::Target,
    ) -> crate::Result<usize> {
        let has_items_above = current_item_idx > LIMIT;
        let has_items_below = current_item_idx < items.len() - LIMIT;

        #[rustfmt::skip]
        let top_ellipsis = if has_items_above { Some("...".paint(ELLIPSIS_STYLE)) } else { None };
        #[rustfmt::skip]
        let bottom_ellipsis = if has_items_below { Some("...".paint(ELLIPSIS_STYLE)) } else { None };

        let nearby_items = items
            .iter()
            .enumerate()
            .filter(|(idx, _)| idx.abs_diff(current_item_idx) <= LIMIT);

        let mut lines_drawn: usize = 0;

        lines_drawn += self.draw_ellipsis(top_ellipsis)?;

        for (idx, item) in nearby_items {
            let maybe_arrow = if idx == current_item_idx { ">" } else { " " };

            writeln!(self.term, "{} {}", maybe_arrow.paint(ARROW_STYLE), item)?;

            lines_drawn += 1;
        }

        lines_drawn += self.draw_ellipsis(bottom_ellipsis)?;

        Ok(lines_drawn)
    }

    fn draw_ellipsis(&mut self, ellipsis: Option<impl std::fmt::Display>) -> crate::Result<usize> {
        if let Some(ellipsis) = ellipsis {
            writeln!(self.term, "{}", ellipsis)?;

            Ok(1)
        } else {
            Ok(0)
        }
    }
}
