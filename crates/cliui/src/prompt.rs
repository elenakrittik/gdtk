use std::{
    fmt::{Display, Write},
    io::Write as IOWrite,
};

use ahash::AHashMap;
use console::Term;
use yansi::Paint;

use crate::prompt::vecview::VecView;

mod action;
mod builders;
mod sentinel;
mod vecview;

const QUESTION_MARK_STYLE: yansi::Style = yansi::Style::new().bright_yellow().bold();
const ARROW_STYLE: yansi::Style = yansi::Style::new().bright_green().bold();
const ACTION_STYLE: yansi::Style = yansi::Style::new().bright_black().bold().dim();
const CHOICE_STYLE: yansi::Style = ARROW_STYLE;
const NO_CHOICE_STYLE: yansi::Style = yansi::Style::new().bright_red().bold();

pub use action::Action;
pub use console::Key;

/// A prompt.
pub struct Prompt<Item, State> {
    question: &'static str,
    term: Term,
    allow_esc: bool,
    view: VecView<Item>,
    actions: AHashMap<Key, Action<Item, State>>,
    pub state: State,
}

impl<Item, State> Prompt<Item, State> {
    pub fn replace_items(&mut self, with: impl Into<Vec<Item>>) -> crate::Result {
        self.view = VecView::new(with.into(), self.view.length, self.view.drag_limit);

        Ok(())
    }
}

impl<Item: Display, State> Prompt<Item, State> {
    pub fn interact(mut self) -> crate::Result<Option<Item>> {
        let mut choice = None;

        self.term.hide_cursor()?;
        self.draw_question()?;

        let mut lines_previously_drawn = self.draw_items()?;

        loop {
            match self.term.read_key()? {
                Key::ArrowUp => {
                    self.view.move_up();
                    self.term.clear_last_lines(lines_previously_drawn)?;

                    lines_previously_drawn = self.draw_items()?;
                }
                Key::ArrowDown => {
                    self.view.move_down();
                    self.term.clear_last_lines(lines_previously_drawn)?;

                    lines_previously_drawn = self.draw_items()?;
                }
                Key::Enter => {
                    choice.replace(self.view.current_idx);
                    break;
                }
                Key::Escape if self.allow_esc => break,
                other => {
                    if let Some(action) = self.actions.get(&other) {
                        (action.callback)(&mut self)?;
                    }

                    self.term.clear_last_lines(lines_previously_drawn)?;
                    lines_previously_drawn = self.draw_items()?;
                }
            }
        }

        self.term.clear_last_lines(lines_previously_drawn)?;
        self.term.clear_last_lines(1)?;
        self.draw_choice(choice)?;
        self.term.show_cursor()?;

        Ok(choice.map(|idx| self.view.items.swap_remove(idx)))
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
        let view = self.view.view();
        let mut idx = self.view.range_start();

        for item in view.items {
            let arrow = if idx == self.view.current_idx {
                ">"
            } else {
                " "
            };

            writeln!(self.term, "{} {}", arrow.paint(ARROW_STYLE), item)?;

            idx += 1;
        }

        let lines_drawn = view.items.len() + self.draw_actions()?;

        Ok(lines_drawn)
    }

    fn draw_choice(&mut self, choice: Option<usize>) -> crate::Result {
        if let Some(choice) = choice {
            writeln!(
                self.term,
                "{} {}: {}",
                '?'.paint(CHOICE_STYLE),
                self.question,
                &self.view.items[choice],
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

    fn draw_actions(&mut self) -> crate::Result<usize> {
        writeln!(self.term)?;

        for (key, action) in &self.actions {
            writeln!(
                self.term,
                "{}{}{} {}",
                '['.paint(ACTION_STYLE),
                display_key(key).paint(ACTION_STYLE),
                ']'.paint(ACTION_STYLE),
                action.description.paint(ACTION_STYLE)
            )?;
        }

        Ok(self.actions.len() + 1)
    }
}

fn display_key(key: &console::Key) -> impl Display + '_ {
    struct DisplayKey<'a>(&'a console::Key);

    impl Display for DisplayKey<'_> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self.0 {
                Key::ArrowLeft => todo!(),
                Key::ArrowRight => todo!(),
                Key::ArrowUp => todo!(),
                Key::ArrowDown => todo!(),
                Key::Enter => todo!(),
                Key::Escape => todo!(),
                Key::Backspace => todo!(),
                Key::Home => todo!(),
                Key::End => todo!(),
                Key::Tab => todo!(),
                Key::BackTab => todo!(),
                Key::Alt => todo!(),
                Key::Del => todo!(),
                Key::Shift => todo!(),
                Key::Insert => todo!(),
                Key::PageUp => todo!(),
                Key::PageDown => todo!(),
                Key::Char(c) => f.write_char(c.to_ascii_uppercase()),
                Key::CtrlC => f.write_str("Ctrl-C"),
                _ => f.write_str("unknown"),
            }
        }
    }

    DisplayKey(key)
}
