pub type Span = std::ops::Range<usize>;

#[derive(Debug)]
pub struct Token<'a> {
    pub kind: TokenKind<'a>,
    pub span: Span,
}

#[derive(Debug)]
pub enum TokenKind<'a> {
    Identifier(&'a str),
}
