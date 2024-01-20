// #![feature(type_alias_impl_trait)]
#![feature(decl_macro)]

use gdtk_ast::poor::{ASTAnnotation, ASTClass, ASTValue, ASTVariable, ASTVariableKind};
use gdtk_lexer::{token::Token, LexOutput};

use crate::error::Error;

pub mod error;

// type TokenIter<'a> = impl Iterator<Item = Token<'a>>;

pub fn parse_file(lexed: LexOutput) -> Result<ASTClass, Error> {
    let (tokens, _diags) = lexed;

    let mut class_name = None;
    let mut extends = None;
    let mut icon = None;
    let mut variables: Vec<ASTVariable<'_>> = vec![];

    let mut ann_stack: Vec<ASTAnnotation<'_>> = vec![];

    let mut iter = tokens.into_iter().map(|v| v.0);

    while let Some(token) = iter.next() {
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
            }
            Token::Const => variables.push(parse_const(&mut iter)),
            Token::Enum => todo!(), //parse_enum(iter),
            Token::Extends => {
                if extends.is_some() {
                    panic!("more than one extends");
                }

                extends = Some(parse_extends(&mut iter));
            }
            Token::Func => todo!(), //parse_func(iter),
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
            }
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
    let arguments = collect_args!(iter, Token::OpeningParenthesis, Token::ClosingParenthesis);

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

pub macro collect_args($iter:expr, $opening:pat, $closing:pat) {{
    expect!($iter, $opening, ());
    collect_args_raw!($iter, $closing)
}}

pub macro collect_args_raw($iter:expr, $closing:pat) {{
    let mut args = vec![];
    let mut expect_comma = false;

    while let Some(token) = $iter.next() {
        match token {
            Token::Comma => {
                if !expect_comma {
                    panic!("unexpected comma, expected a value");
                }
                expect_comma = false;
            }
            Token::Blank(_) => (),
            $closing => break,
            other => {
                if expect_comma {
                    panic!("expected comma, got {other:?}");
                }
                args.push(parse_value($iter, Some(other)));
                expect_comma = true;
            }
        }
    }

    args
}}

pub fn parse_const<'a, T>(iter: &mut T) -> ASTVariable<'a>
where
    T: Iterator<Item = Token<'a>>,
{
    expect!(iter, Token::Blank(_), ());
    let identifier = expect_blank_prefixed!(iter, Token::Identifier(s), s);

    let mut typehint = None;
    let mut infer_type = false;

    // either colon or an assignment
    let value = match next_non_blank!(iter) {
        // got a colon, has to be followed by an identifier (type hint) or an assignment
        Token::Colon => {
            match next_non_blank!(iter) {
                Token::Identifier(s) => {
                    typehint = Some(s);
                    expect_blank_prefixed!(iter, Token::Assignment, ());
                    parse_value(iter, None)
                },
                Token::Assignment => {
                    infer_type = true;
                    parse_value(iter, None)
                }
                other => panic!("unexpected {other:?}, expected identifier or assignment"),
            }
        },
        Token::Assignment => parse_value(iter, None),
        other => panic!("unexpected {other:?}, expected colon or assignment"),
    };

    ASTVariable {
        identifier,
        infer_type,
        typehint,
        value: Some(value),
        kind: ASTVariableKind::Constant,
    }
}

pub fn parse_var<'a, T>(iter: &mut T) -> ASTVariable<'a>
where
    T: Iterator<Item = Token<'a>>,
{
    expect!(iter, Token::Blank(_), ());
    let identifier = expect_blank_prefixed!(iter, Token::Identifier(s), s);

    let mut infer_type = false;
    let mut typehint = None;
    let mut value = None;

    // a colon, an assignment or a newline
    match next_non_blank!(iter) {
        Token::Colon => {
            // colon can be followed by an identifier (typehint) or an assignment (means the type should be inferred)
            match next_non_blank!(iter) {
                Token::Identifier(s) => {
                    // we got the typehint
                    typehint = Some(s);

                    // typehint can be followed by an assignment or a newline
                    match next_non_blank!(iter) {
                        // found assignment, then there must be a value
                        Token::Assignment => value = Some(parse_value(iter, None)),
                        // no value
                        Token::Newline => (),
                        other => panic!("unexpected {other:?}, expected assignment or newline"),
                    }
                },
                Token::Assignment => {
                    infer_type = true;
                    value = Some(parse_value(iter, None));
                },
                other => panic!("unexpected {other:?}, expected assignment or newline"),
            }
        },
        Token::Assignment => value = Some(parse_value(iter, None)),
        Token::Newline => (), // both typehint and value are optional
        other => panic!("unexpected {other:?}, expected colon, assignment or newline"),
    }

    ASTVariable {
        identifier,
        infer_type: false, // TODO
        typehint,
        value,
        kind: ASTVariableKind::Regular,
    }
}

pub fn parse_extends<'a, T>(iter: &mut T) -> &'a str
where
    T: Iterator<Item = Token<'a>>,
{
    expect!(iter, Token::Blank(_), ());
    expect_blank_prefixed!(iter, Token::Identifier(s), s)
}

pub fn parse_value<'a, T>(iter: &mut T, mut token: Option<Token<'a>>) -> ASTValue<'a>
where
    T: Iterator<Item = Token<'a>>,
{
    if token.is_none() {
        token = Some(next_non_blank!(iter));
    }

    match token.unwrap() {
        Token::Identifier(s) => ASTValue::Identifier(s),
        Token::Integer(i) => ASTValue::Number(i),
        Token::BinaryInteger(i) => ASTValue::Number(i as i64),
        Token::HexInteger(i) => ASTValue::Number(i as i64),
        Token::ScientificFloat(f) => ASTValue::Float(f),
        Token::Float(f) => ASTValue::Float(f),
        Token::String(s) => ASTValue::String(s),
        Token::StringName(s) => ASTValue::StringName(s),
        Token::Node(s) => ASTValue::Node(s),
        Token::UniqueNode(s) => ASTValue::UniqueNode(s),
        Token::NodePath(s) => ASTValue::NodePath(s),
        Token::Boolean(b) => ASTValue::Boolean(b),
        Token::OpeningBracket => ASTValue::Array(collect_args_raw!(iter, Token::ClosingBracket)),
        other => panic!("unknown or unsupported expression: {other:?}"),
    }
}
