/// An argument.
#[derive(Debug)]
pub enum Arg<'a> {
    /// A short argument.
    Short(char),
    /// A long argument.
    Long(&'a str),
    /// A value.
    Value(&'a str),
}

#[derive(Debug)]
enum State<'a> {
    None,
    Short(u8, &'a str),
    TreatAsValues,
}

/// A command-line argument parser.
#[derive(Debug)]
pub struct Parser<'a> {
    args: std::env::Args,
    state: State<'a>,
}

impl<'a> Parser<'a> {
    /// Creates a new `Parser` from the environment.
    pub fn from_env() -> Self {
        Self {
            args: std::env::args(),
            state: State::None,
        }
    }

    pub fn switch(&mut self) -> Option<<Self as Iterator>::Item> {
        self.state = State::None;

        let arg = self.args.next()?;

        if arg.starts_with("--") {
            // a `--` encountered
            if arg[2..].trim().is_empty() {
                self.state = State::TreatAsValues;

                return Some(Arg::Value(&self.args.next()?));
            }

            return Some(Arg::Long(&arg[2..]));
        }

        if arg.starts_with('-') {
            self.state = State::Short(2, &arg);

            return Some(Arg::Short(arg.chars().nth(1)?));
        }

        Some(Arg::Value(&arg))
    }
}

impl<'a> Iterator for Parser<'a> {
    type Item = Arg<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.state {
            State::None => todo!(),
            State::Short(idx, short) => todo!(),
        }
    }
}
