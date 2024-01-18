// #![feature(type_alias_impl_trait)]
#![feature(decl_macro)]

use gdtk_ast::poor::{ASTAnnotation, ASTClass, ASTValue, ASTVariable, ASTVariableKind};
use gdtk_lexer::{token::Token, LexOutput};

use crate::error::Error;

pub mod error;

// type TokenIter<'a> = impl Iterator<Item = Token<'a>>;

pub fn parse_file<'a>(lexed: LexOutput<'a>) -> Result<ASTClass, Error> {
    let (tokens, _diags) = lexed;

    let mut class_name = None;
    let mut extends = None;
    let mut icon = None;
    let mut variables: Vec<ASTVariable<'_>> = vec![];

    let mut ann_stack: Vec<ASTAnnotation<'_>> = vec![];

    let mut iter = tokens.into_iter().map(|v| v.0);

    loop {
        let token = match iter.next() {
            Some(tkn) => tkn,
            None => break,
        };

        dbg!(&token);

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
            Token::Enum => todo!(),    //parse_enum(iter),
            Token::Extends => {
                if extends.is_some() {
                    panic!("more than one extends");
                }

                extends = Some(parse_extends(&mut iter));
            },
            Token::Func => todo!(),    //parse_func(iter),
            Token::In => todo!(),
            Token::Is => todo!(),
            Token::Signal => todo!(), //parse_signal(iter),
            Token::Static => todo!(), //variables.push(parse_static(iter)),
            Token::Var => variables.push(parse_var(&mut iter)),
            Token::Annotation => {
                let ann = parse_annotation(&mut iter);

                if ann.identifier == "icon" && matches!(class_name, None) {
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

    if ann_stack.len() > 0 {
        panic!("unapplied annotations: {ann_stack:?}");
    }

    Ok(ASTClass {
        class_name,
        extends,
        icon,
        variables,
        enums: vec![],
        functions: vec![],
    })
}

pub fn parse_classname<'a, T>(iter: &mut T) -> &'a str
where
    T: Iterator<Item = Token<'a>>,
{
    expect_blank_prefixed!(iter, Token::Identifier(i), i)
}

pub fn parse_annotation<'a, T>(iter: &mut T) -> ASTAnnotation<'a>
where
    T: Iterator<Item = Token<'a>>,
{
    let identifier = expect_blank_prefixed!(iter, Token::Identifier(i), i);
    let arguments = collect_args(iter);

    ASTAnnotation {
        identifier,
        arguments,
    }
}

pub macro expect($iter:expr, $variant:pat, $ret:expr) {
    match $iter.next() {
        Some($variant) => $ret,
        _ => panic!("expected {{__macro_arg1}}"),
    }
}

pub macro expect_blank_prefixed($iter:expr, $variant:pat, $ret:expr) {
    loop {
        if let Some(token) = $iter.next() {
            match token {
                Token::Blank(_) => (),
                $variant => break $ret,
                _ => panic!("expected {{__macro_arg1}}"),
            }
        } else {
            panic!("unexpected EOF");
        }
    }
}

pub macro next_non_blank($iter:expr) {
    loop {
        if let Some(token) = $iter.next() {
            match token {
                Token::Blank(_) => (),
                other => break other,
            }
        } else {
            panic!("unexpected EOF");
        }
    }
}

pub fn collect_args<'a, T>(iter: &mut T) -> Vec<ASTValue<'a>>
where
    T: Iterator<Item = Token<'a>>,
{
    expect!(iter, Token::OpeningParenthesis, ());

    let mut args = vec![];
    let mut expect_comma = false;

    while let Some(token) = iter.next() {
        match token {
            Token::String(s) => {
                if expect_comma {
                    panic!("expected comma, got string {:?}", s);
                }
                args.push(ASTValue::String(s));
                expect_comma = true;
            },
            Token::Comma => {
                if !expect_comma {
                    panic!("unexpected comma");
                }
                expect_comma = false;
            },
            Token::Blank(_) => (),
            Token::ClosingParenthesis => break,
            other => panic!("unexpected {other:?}"),
        }
    }

    args
}

pub fn parse_const<'a, T>(iter: &mut T) -> ASTVariable<'a>
where
    T: Iterator<Item = Token<'a>>
{
    expect!(iter, Token::Blank(_), ());
    let identifier = expect_blank_prefixed!(iter, Token::Identifier(s), s);
    expect_blank_prefixed!(iter, Token::Colon, ());
    let typehint = expect_blank_prefixed!(iter, Token::Identifier(s), Some(s)); // TODO: support subscription
    expect_blank_prefixed!(iter, Token::Assignment, ());
    let value = expect_blank_prefixed!(iter, Token::Integer(i), Some(ASTValue::Number(i))); // TODO: parse value

    ASTVariable {
        identifier,
        typehint,
        value,
        kind: ASTVariableKind::Constant,
    }
}

pub fn parse_var<'a, T>(iter: &mut T) -> ASTVariable<'a>
where
    T: Iterator<Item = Token<'a>>,
{
    expect!(iter, Token::Blank(_), ());
    let identifier = expect_blank_prefixed!(iter, Token::Identifier(s), s);

    let mut typehint = None;
    let mut value = None;

    match next_non_blank!(iter) {
        Token::Colon => {
            typehint = expect_blank_prefixed!(iter, Token::Identifier(s), Some(s));

            match next_non_blank!(iter) {
                Token::Assignment => value = expect_blank_prefixed!(iter, Token::Integer(i), Some(ASTValue::Number(i))),
                Token::Newline => (),
                other => panic!("unexpected {other:?}, expected assignment or newline"),
            }
        },
        Token::Assignment => value = expect_blank_prefixed!(iter, Token::Integer(i), Some(ASTValue::Number(i))),
        Token::Newline => (),
        other => panic!("unexpected {other:?}, expected colon, assignment or newline"),
    }

    ASTVariable {
        identifier,
        typehint,
        value,
        kind: ASTVariableKind::Regular,
    }
}

pub fn parse_extends<'a, T>(iter: &mut T) -> &'a str
where
    T: Iterator<Item = Token<'a>>
{
    expect!(iter, Token::Blank(_), ());
    expect_blank_prefixed!(iter, Token::Identifier(s), s)
}
