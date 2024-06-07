use std::{fmt, io};

use console::Term;
#[cfg(feature = "fuzzy-select")]
use fuzzy_matcher::skim::SkimMatcherV2;

use crate::{theme::Theme, Result};

/// Helper struct to conveniently render a theme.
pub(crate) struct TermThemeRenderer<'a> {
    term: &'a Term,
    theme: &'a dyn Theme,
    height: usize,
    prompt_height: usize,
    prompts_reset_height: bool,
}

impl<'a> TermThemeRenderer<'a> {
    pub fn new(term: &'a Term, theme: &'a dyn Theme) -> TermThemeRenderer<'a> {
        TermThemeRenderer {
            term,
            theme,
            height: 0,
            prompt_height: 0,
            prompts_reset_height: true,
        }
    }

    fn write_formatted_line<
        F: FnOnce(&mut TermThemeRenderer, &mut dyn fmt::Write) -> fmt::Result,
    >(
        &mut self,
        f: F,
    ) -> Result {
        let mut buf = String::new();
        f(self, &mut buf).map_err(|err| io::Error::new(io::ErrorKind::Other, err))?;
        self.height += buf.chars().filter(|&x| x == '\n').count() + 1;
        Ok(self.term.write_line(&buf)?)
    }

    fn write_formatted_prompt<
        F: FnOnce(&mut TermThemeRenderer, &mut dyn fmt::Write) -> fmt::Result,
    >(
        &mut self,
        f: F,
    ) -> Result {
        self.write_formatted_line(f)?;
        if self.prompts_reset_height {
            self.prompt_height = self.height;
            self.height = 0;
        }
        Ok(())
    }

    #[cfg(feature = "fuzzy-select")]
    pub fn fuzzy_select_prompt(
        &mut self,
        prompt: &str,
        search_term: &str,
        cursor_pos: usize,
    ) -> Result {
        self.write_formatted_prompt(|this, buf| {
            this.theme
                .format_fuzzy_select_prompt(buf, prompt, search_term, cursor_pos)
        })
    }

    pub fn input_prompt_selection(&mut self, prompt: &str, sel: &str) -> Result {
        self.write_formatted_prompt(|this, buf| {
            this.theme.format_input_prompt_selection(buf, prompt, sel)
        })
    }

    #[cfg(feature = "fuzzy-select")]
    pub fn fuzzy_select_prompt_item(
        &mut self,
        text: &str,
        active: bool,
        highlight: bool,
        matcher: &SkimMatcherV2,
        search_term: &str,
    ) -> Result {
        self.write_formatted_line(|this, buf| {
            this.theme.format_fuzzy_select_prompt_item(
                buf,
                text,
                active,
                highlight,
                matcher,
                search_term,
            )
        })
    }

    pub fn clear(&mut self) -> Result {
        self.term
            .clear_last_lines(self.height + self.prompt_height)?;
        self.height = 0;
        self.prompt_height = 0;
        Ok(())
    }

    pub fn clear_preserve_prompt(&mut self, size_vec: &[usize]) -> Result {
        let mut new_height = self.height;
        let prefix_width = 2;
        //Check each item size, increment on finding an overflow
        for size in size_vec {
            if *size > self.term.size().1 as usize {
                new_height += (((*size as f64 + prefix_width as f64) / self.term.size().1 as f64)
                    .ceil()) as usize
                    - 1;
            }
        }

        self.term.clear_last_lines(new_height)?;
        self.height = 0;
        Ok(())
    }
}
