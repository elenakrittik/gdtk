#[derive(Debug, PartialEq)]
pub struct ASTModule {
    pub statements: Vec<ASTStatement>,
}

#[derive(Debug, PartialEq)]
pub enum ASTStatement {
    Comment(String),
    ClassName(String),
    Extends(String),
    // 0 - stmt, 1 - comm. Boxes are essentially unneeded but unfortunately rustc doesn't think so
    Commented(Box<ASTStatement>, Box<ASTStatement>),
    Value(ASTValue),
}

#[derive(Debug, PartialEq)]
pub enum ASTValue {
    Int(i64),
}
