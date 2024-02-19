/// WIP state-based parser. Currently unused.
use gdtk_ast::poor::ASTStatement;
use gdtk_lexer::Token;

#[derive(Debug)]
pub enum State {
    /// Parse a top-level item.
    TopLevel,
}

#[derive(Debug)]
pub struct ParserStateMachine<'a, T>
where
    T: Iterator<Item = Token<'a>>,
{
    pub state: State,
    pub iter: T,
}

impl<'a, T> ParserStateMachine<'a, T>
where
    T: Iterator<Item = Token<'a>>,
{
    pub fn new(iter: T) -> Self {
        Self {
            state: State::TopLevel,
            iter,
        }
    }

    pub fn next(&mut self) -> ASTStatement<'a> {
        match self.state {
            State::TopLevel => todo!(),
        }
    }
}
