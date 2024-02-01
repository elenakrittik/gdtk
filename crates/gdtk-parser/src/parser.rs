use gdtk_ast::poor::ASTStatement;
use gdtk_lexer::Token;

#[derive(Debug)]
pub enum State {
    /// Parse a top-level item.
    TopLevel,
}

#[derive(Debug)]
pub struct ParserStateMachine<'a> {
    pub state: State,
    pub iter: impl Iterator<Item = Token<'a>>,
}

impl<'a> ParserStateMachine<'a> {
    pub fn new(iter: impl Iterator<Item = Token<'a>>) -> Self {
        Self { state: State::TopLevel, iter }
    }

    pub fn next() -> ASTStatement<'a> {
        match self.state {
            State::TopLevel => todo!(),
            
        }
    }
}
