use crate::arg::Arg;

#[derive(Debug)]
enum State {
    /// No special state.
    None,
    /// The parser is in the process of parsing a short argument.
    /// `0` is the index of the next character to process, `1` is
    /// the short argument itself.
    Short(usize, String),
    /// The parser has encountered a `--` and therefore anything and.
    /// everything shall be treated as values.
    TreatAsValues,
}

/// A command-line argument parser.
#[derive(Debug)]
pub struct Parser {
    args: std::env::Args,
    state: State,
}

impl Parser {
    /// Create a new `Parser` from the environment.
    pub fn from_env() -> Self {
        Self {
            args: std::env::args(),
            state: State::None,
        }
    }

    /// Reset state to [State::None] and parse the next argument.
    fn switch(&mut self) -> Option<<Self as Iterator>::Item> {
        self.state = State::None;

        let arg = self.args.next()?;

        if let Some(arg) = arg.strip_prefix("--") {
            // encountered `--`, treat the remaining os arguments as values
            if arg.trim().is_empty() {
                self.state = State::TreatAsValues;

                return Some(Arg::Value(self.args.next()?));
            }

            return Some(Arg::Long(arg.to_string()));
        }

        if arg.starts_with('-') {
            // the 0th char is a `-`, the next (1st) char is the first
            // short argument
            const FIRST_SHORT: usize = 1;

            let chr = arg.chars().nth(FIRST_SHORT)?;

            self.state = State::Short(FIRST_SHORT + 1, arg);

            return Some(Arg::Short(chr));
        }

        Some(Arg::Value(arg))
    }
}

impl Iterator for Parser {
    type Item = Arg;

    fn next(&mut self) -> Option<Self::Item> {
        Some(match self.state {
            State::None => self.switch()?,
            State::Short(ref mut idx, ref short) => {
                if let Some(chr) = short.chars().nth(*idx) {
                    // if the short string is not "exhausted" yet, ++ the
                    // pointer and return the short argument
                    *idx += 1;

                    Arg::Short(chr)
                } else {
                    // if it *is* exhausted, go on to the next os argument
                    self.switch()?
                }
            }
            State::TreatAsValues => Arg::Value(self.args.next()?),
        })
    }
}
