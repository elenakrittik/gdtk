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
    state: State,
}

impl<Item: Display, State> Prompt<Item, State> {
    pub fn interact(mut self) -> crate::Result<(Option<Item>, State)> {
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
                    choice.replace(self.view.current_item_index());
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

        let item = choice.map(|idx| self.view.consume_item(idx));

        Ok((item, self.state))
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
            let arrow = if idx == self.view.current_item_index() {
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
                &self.view[choice],
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

    #[inline]
    pub fn actions(&self) -> &AHashMap<Key, Action<Item, State>> {
        &self.actions
    }

    #[inline]
    pub fn actions_mut(&mut self) -> &mut AHashMap<Key, Action<Item, State>> {
        &mut self.actions
    }

    #[inline]
    pub fn state(&self) -> &State {
        &self.state
    }

    #[inline]
    pub fn state_mut(&mut self) -> &mut State {
        &mut self.state
    }
}

fn display_key(key: &console::Key) -> impl Display + '_ {
    struct DisplayKey<'a>(&'a console::Key);

    impl Display for DisplayKey<'_> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(match self.0 {
                Key::ArrowLeft => "Arrow-Left",
                Key::ArrowRight => "Arrow-Right",
                Key::ArrowUp => "Arrow-Up (inactive)",
                Key::ArrowDown => "Arrow-Down (inactive)",
                Key::Enter => "Enter (inactive)",
                Key::Escape => "Esc (inactive)",
                Key::Backspace => "Backspace",
                Key::Home => "Home",
                Key::End => "End",
                Key::Tab => "Tab",
                Key::BackTab => "Shift-Tab",
                Key::Alt => "Alt",
                Key::Del => "Del",
                Key::Shift => "Shift",
                Key::Insert => "Insert",
                Key::PageUp => "Page-Up",
                Key::PageDown => "Page-Down",
                Key::Char(c) => return f.write_char(c.to_ascii_uppercase()),
                Key::CtrlC => "Ctrl-C",
                _ => "unknown",
            })
        }
    }

    DisplayKey(key)
}
