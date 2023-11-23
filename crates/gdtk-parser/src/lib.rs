// #![feature(type_alias_impl_trait)]

use gdtk_ast::poor::ASTClass;
use gdtk_lexer::{token::Token, LexOutput};

use crate::error::Error;

pub mod error;

// type TokenIter<'a> = impl Iterator<Item = Token<'a>>;

pub fn parse_file<'a>(lexed: LexOutput<'a>) -> Result<ASTClass, Error> {
    let (tokens, _diags) = lexed;
    dbg!(&tokens);

    let mut class_name = None;

    let mut iter = tokens.into_iter().map(|v| v.0);

    loop {
        let token = match iter.next() {
            Some(tkn) => tkn,
            None => break,
        };

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

                (class_name, iter) = parse_classname(iter)
            },
            Token::Const => {
                if class_name.is_some() {
                    panic!("more than one class_name")
                }

                (class_name, iter) = parse_classname(iter)
            },
            Token::Enum => todo!(),    //parse_enum(iter),
            Token::Extends => todo!(), //parse_extend(iter),
            Token::Func => todo!(),    //parse_func(iter),
            Token::In => todo!(),
            Token::Is => todo!(),
            Token::Signal => todo!(),     //parse_signal(iter),
            Token::Static => todo!(),     //parse_static(iter),
            Token::Var => todo!(),        //parse_var(iter),
            Token::Annotation => todo!(), //parse_annotation(iter),
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
            Token::Newline => todo!(),
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

    Ok(ASTClass {
        class_name: class_name,
        extends: None,
        icon_path: None,
        variables: vec![],
        enums: vec![],
        functions: vec![],
    })
}

pub fn parse_classname<'a, T>(mut iter: T) -> (Option<&'a str>, T)
where
    T: Iterator<Item = Token<'a>>,
{
    let blank = match iter.next() {
        Some(t) => t,
        None => panic!("expected at least one space after class_name"),
    };

    if !matches!(blank, Token::Blank(_)) {
        panic!("expected at least one space after class_name")
    }

    let mut ident = None;

    while let Some(token) = iter.next() {
        match token {
            Token::Blank(_) => (),
            Token::Identifier(i) => {
                ident = Some(i);
                break;
            }
            other => panic!("expected identifier after class_name, found {:?}", other),
        }
    }

    if matches!(ident, None) {
        panic!("expected identifier after class_name");
    }

    (ident, iter)
}
