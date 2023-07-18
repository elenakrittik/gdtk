use super::values::value;
use crate::ast::ASTStatement;

pub fn statement(line: &str) -> anyhow::Result<ASTStatement> {
    let parser = sparsec::Sparsec::new(line);

    Ok(ASTStatement::Value(value(&parser)?))
}

// pub fn statements<Input>() -> impl Parser<Input, Output = Vec<ASTStatement>>
// where
//     Input: Stream<Token = char>,
// {
//     many(
//         (statement(), optional(comment()), newlines().silent()).map(|(stmt, comm, _)| {
//             if let Some(cmt) = comm {
//                 ASTStatement::Commented(Box::new(stmt), Box::new(cmt))
//             } else {
//                 stmt
//             }
//         }),
//     )
// }

// pub fn statement<Input>() -> impl Parser<Input, Output = ASTStatement>
// where
//     Input: Stream<Token = char>,
// {
//     choice((
//         classname_statement(),
//         extends_statement(),
//         comment(),
//         value_statement(),
//     ))
// }

// pub fn classname_statement<Input>() -> impl Parser<Input, Output = ASTStatement>
// where
//     Input: Stream<Token = char>,
// {
//     simple_statement("class_name ").map(|(_, _, ident, _)| ASTStatement::ClassName(ident))
// }

// pub fn extends_statement<Input>() -> impl Parser<Input, Output = ASTStatement>
// where
//     Input: Stream<Token = char>,
// {
//     simple_statement("extends ").map(|(_, _, ident, _)| ASTStatement::Extends(ident))
// }

// pub fn value_statement<Input>() -> impl combine::Parser<Input, Output = ASTStatement>
// where
//     Input: combine::Stream<Token = char>,
// {
//     (value(), spaces()).map(|(val, _)| ASTStatement::Value(val))
// }
