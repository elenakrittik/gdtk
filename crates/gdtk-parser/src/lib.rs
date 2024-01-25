#![feature(decl_macro)]

use gdtk_ast::poor::{ASTAnnotation, ASTClass, ASTVariable, ASTFunction, ASTEnum};
use gdtk_lexer::{token::Token, LexOutput};

pub use crate::error::Error;

use crate::functions::parse_func;
use crate::classes::{parse_classname, parse_extends, parse_enum};
use crate::misc::parse_annotation;
use crate::variables::{parse_const, parse_var};

pub mod error;
pub mod functions;
pub mod classes;
pub mod misc;
pub mod utils;
pub mod variables;
pub mod values;

pub fn parse_file(lexed: LexOutput) -> Result<ASTClass, Error> {
    let (tokens, _diags) = lexed;

    let mut class_name = None;
    let mut extends = None;
    let mut icon = None;
    let mut variables: Vec<ASTVariable<'_>> = vec![];
    let mut enums: Vec<ASTEnum<'_>> = vec![];
    let mut functions: Vec<ASTFunction<'_>> = vec![];

    let mut ann_stack: Vec<ASTAnnotation<'_>> = vec![];

    let mut iter = tokens.into_iter().map(|v| v.0);

    while let Some(token) = iter.next() {
        match token {
            Token::If => todo!(),
            Token::Elif => todo!(),
            Token::Else => todo!(),
            Token::For => todo!(),
            Token::While => todo!(),
            Token::Break => todo!(),
            Token::Continue => todo!(),
            Token::Pass => todo!(),
            Token::Return => todo!(),
            Token::Match => todo!(),
            Token::As => todo!(),
            Token::Assert => todo!(),
            Token::Await => todo!(),
            Token::Breakpoint => todo!(),
            Token::Class => todo!(),
            Token::ClassName => {
                if class_name.is_some() {
                    panic!("more than one class_name")
                }

                class_name = Some(parse_classname(&mut iter));
            },
            Token::Const => variables.push(parse_const(&mut iter)),
            Token::Enum => enums.push(parse_enum(&mut iter)),
            Token::Extends => {
                if extends.is_some() {
                    panic!("more than one extends");
                }

                extends = Some(parse_extends(&mut iter));
            },
            Token::Func => functions.push(parse_func(&mut iter)),
            Token::In => todo!(),
            Token::Is => todo!(),
            Token::Signal => todo!(), //parse_signal(iter),
            Token::Static => todo!(), //variables.push(parse_static(iter)),
            Token::Var => variables.push(parse_var(&mut iter)),
            Token::Annotation => {
                let ann = parse_annotation(&mut iter);

                if ann.identifier == "icon" && class_name.is_none() {
                    icon = Some(ann);
                } else {
                    ann_stack.push(ann);
                }
            },
            Token::OpeningParenthesis => todo!(),
            Token::ClosingParenthesis => todo!(),
            Token::OpeningBracket => todo!(),
            Token::ClosingBracket => todo!(),
            Token::OpeningBrace => todo!(),
            Token::ClosingBrace => todo!(),
            Token::Comma => todo!(),
            Token::Semicolon => todo!(),
            Token::Period => todo!(),
            Token::Range => todo!(),
            Token::Colon => todo!(),
            Token::Dollar => todo!(),
            Token::Arrow => todo!(),
            Token::Newline => (),
            Token::Indent => todo!(),
            Token::Dedent => todo!(),
            Token::Spaces => todo!(),
            Token::Blank(_) => (),
            Token::Comment(_) => (),
            Token::Namespace => todo!(),
            Token::Trait => todo!(),
            Token::Yield => todo!(),
            _ => panic!("not allowed"),
        }
    }

    if !ann_stack.is_empty() {
        panic!("unapplied annotations: {ann_stack:?}");
    }

    Ok(ASTClass {
        class_name,
        extends,
        icon,
        variables,
        enums,
        functions,
    })
}
