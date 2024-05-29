pub type Parser = lexopt::Parser;

pub trait Command {
    type Output = ();
    type Error = !;

    fn from_env() -> Result<Self, Self::Error> {
        let mut parser = Parser::from_env();

        self.parse(&mut parser)
    }

    fn parse(parser: &mut Parser) -> Result<Self, Self::Error>;
    fn run(self) -> Result<Self::Output, Self::Error>;
}
