#![feature(decl_macro)]

use gdtk_ast::poor::{ASTAnnotation, ASTClass, ASTEnum, ASTFunction, ASTVariable};
use gdtk_lexer::{token::TokenKind, LexOutput};

use crate::classes::{parse_classname, parse_enum, parse_extends};
pub use crate::error::Error;
use crate::functions::parse_func;
use crate::misc::parse_annotation;
use crate::variables::{parse_const, parse_var};

pub mod classes;
pub mod error;
pub mod functions;
pub mod parser;
pub mod misc;
pub mod utils;
pub mod values;
pub mod variables;

pub fn parse_file(lexed: LexOutput) -> Result<ASTClass, Error> {
    let (tokens, _diags) = lexed;

    let mut class_name = None;
    let mut extends = None;
    let mut icon = None;
    let mut variables: Vec<ASTVariable<'_>> = vec![];
    let mut enums: Vec<ASTEnum<'_>> = vec![];
    let mut functions: Vec<ASTFunction<'_>> = vec![];

    let mut ann_stack: Vec<ASTAnnotation<'_>> = vec![];

    let mut iter = tokens.into_iter();

    while let Some(token) = iter.next() {
        match token.kind {
            TokenKind::If => todo!(),
            TokenKind::Elif => todo!(),
            TokenKind::Else => todo!(),
            TokenKind::For => todo!(),
            TokenKind::While => todo!(),
            TokenKind::Break => todo!(),
            TokenKind::Continue => todo!(),
            TokenKind::Pass => todo!(),
            TokenKind::Return => todo!(),
            TokenKind::Match => todo!(),
            TokenKind::As => todo!(),
            TokenKind::Assert => todo!(),
            TokenKind::Await => todo!(),
            TokenKind::Breakpoint => todo!(),
            TokenKind::Class => todo!(),
            TokenKind::ClassName => {
                if class_name.is_some() {
                    panic!("more than one class_name")
                }

                class_name = Some(parse_classname(&mut iter));
            }
            TokenKind::Const => variables.push(parse_const(&mut iter)),
            TokenKind::Enum => enums.push(parse_enum(&mut iter)),
            TokenKind::Extends => {
                if extends.is_some() {
                    panic!("more than one extends");
                }

                extends = Some(parse_extends(&mut iter));
            }
            TokenKind::Func => functions.push(parse_func(&mut iter)),
            TokenKind::In => todo!(),
            TokenKind::Is => todo!(),
            TokenKind::Signal => todo!(), //parse_signal(iter),
            TokenKind::Static => todo!(), //variables.push(parse_static(iter)),
            TokenKind::Var => variables.push(parse_var(&mut iter)),
            TokenKind::Annotation => {
                let ann = parse_annotation(&mut iter);

                if ann.identifier == "icon" && class_name.is_none() {
                    icon = Some(ann);
                } else {
                    ann_stack.push(ann);
                }
            }
            TokenKind::OpeningParenthesis => todo!(),
            TokenKind::ClosingParenthesis => todo!(),
            TokenKind::OpeningBracket => todo!(),
            TokenKind::ClosingBracket => todo!(),
            TokenKind::OpeningBrace => todo!(),
            TokenKind::ClosingBrace => todo!(),
            TokenKind::Comma => todo!(),
            TokenKind::Semicolon => todo!(),
            TokenKind::Period => todo!(),
            TokenKind::Range => todo!(),
            TokenKind::Colon => todo!(),
            TokenKind::Dollar => todo!(),
            TokenKind::Arrow => todo!(),
            TokenKind::Newline => (),
            TokenKind::Indent => todo!(),
            TokenKind::Dedent => todo!(),
            TokenKind::Spaces => todo!(),
            TokenKind::Blank(_) => (),
            TokenKind::Comment(_) => (),
            TokenKind::Namespace => todo!(),
            TokenKind::Trait => todo!(),
            TokenKind::Yield => todo!(),
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
