use crate::Parser;

/// A command.
#[allow(async_fn_in_trait)]
pub trait Command: Sized {
    /// The result of running the command.
    type Output = ();
    /// The error type for the command.
    type Error = !;

    /// Parse the command from the environment. Commonly used to parse
    /// the root command.
    fn from_env() -> Result<Self, Self::Error> {
        let mut parser = Parser::from_env();

        Self::parse(&mut parser)
    }

    /// Parse the command.
    fn parse(parser: &mut Parser) -> Result<Self, Self::Error>;

    /// Run the command.
    fn run(self) -> Result<Self::Output, Self::Error>;
}
