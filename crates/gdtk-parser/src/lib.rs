// #![feature(type_alias_impl_trait)]
#![feature(decl_macro)]

use gdtk_ast::poor::{
    ASTAnnotation, ASTClass, ASTEnum, ASTEnumVariant, ASTFunction, ASTFunctionParameter, ASTValue,
    ASTVariable, ASTVariableKind, CodeBlock, DictValue,
};
use gdtk_lexer::{token::Token, LexOutput};

use crate::error::Error;

pub mod error;

// TODO: some expressions of form "expect!(iter, Token::Blank(_), ())" may be unnecessary
// due to guarantees given by the lexer

// type TokenIter<'a> = impl Iterator<Item = Token<'a>>;

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
            }
            Token::Const => variables.push(parse_const(&mut iter)),
            Token::Enum => enums.push(parse_enum(&mut iter)),
            Token::Extends => {
                if extends.is_some() {
                    panic!("more than one extends");
                }

                extends = Some(parse_extends(&mut iter));
            }
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
        enums,
        functions,
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
        other => panic!("expected {}, found {other:?}", stringify!($variant)),
    }
}

pub macro expect_blank_prefixed($iter:expr, $variant:pat, $ret:expr) {
    loop {
        if let Some(token) = $iter.next() {
            match token {
                Token::Blank(_) => (),
                $variant => break $ret,
                other => panic!("expected {}, found {other:?}", stringify!($variant)),
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
                }
                // infer type
                Token::Assignment => {
                    infer_type = true;
                    parse_value(iter, None)
                }
                other => panic!("unexpected {other:?}, expected identifier or assignment"),
            }
        }
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
    let (identifier, infer_type, typehint, value) = parse_idtydef!(iter, Token::Newline => (),);

    ASTVariable {
        identifier,
        infer_type,
        typehint,
        value,
        kind: ASTVariableKind::Regular,
    }
}

pub macro parse_idtydef($iter:expr, $($endpat:pat => $endcode:expr,)*) {{
    let identifier = expect_blank_prefixed!($iter, Token::Identifier(s), s);

    dbg!(&identifier);

    let mut infer_type = false;
    let mut typehint = None;
    let mut value = None;

    // a colon, an assignment or a newline
    match next_non_blank!($iter) {
        Token::Colon => {
            eprintln!("got colon");
            // colon can be followed by an identifier (typehint) or an assignment (means the type should be inferred)
            match next_non_blank!($iter) {
                Token::Identifier(s) => {
                    eprintln!("got identifier/typehint");
                    // we got the typehint
                    typehint = Some(s);

                    // typehint can be followed by an assignment or a newline
                    match next_non_blank!($iter) {
                        // found assignment, then there must be a value
                        Token::Assignment => {
                            dbg!("expecting value");
                            value = Some(parse_value($iter, None));
                            dbg!("got value: {}", &value);
                        },
                        // no value
                        $($endpat => $endcode,)*
                        other => panic!("unexpected {other:?}, expected assignment or newline"),
                    }
                },
                Token::Assignment => {
                    dbg!("got assignment => infer type");
                    infer_type = true;
                    dbg!("expecting value");
                    value = Some(parse_value($iter, None));
                    dbg!("got value: {}", &value);
                },
                other => panic!("unexpected {other:?}, expected assignment or newline"),
            }
        },
        Token::Assignment => {
            dbg!("got assignment, expecting value");
            value = Some(parse_value($iter, None));
            dbg!("got value: {}", &value);
        },
        $($endpat => $endcode,)*
        other => panic!("unexpected {other:?}, expected colon, assignment or newline"),
    }

    (identifier, infer_type, typehint, value)
}}

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
        Token::Identifier(s) => {
            match next_non_blank!(iter) {
                Token::OpeningParenthesis => ASTValue::Call(
                    Box::new(ASTValue::Identifier(s)),
                    collect_args_raw!(iter, Token::ClosingParenthesis),
                ),
                Token::Newline => ASTValue::Identifier(s),
                // TODO: prop access
                other => panic!("unexpected {other:?}, expected parenthesis"),
            }
        }
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
        Token::OpeningBrace => ASTValue::Dictionary(parse_dictionary(iter)),
        Token::Minus => match parse_value(iter, None) {
            ASTValue::Number(n) => ASTValue::Number(-n),
            ASTValue::Float(f) => ASTValue::Float(f),
            _ => panic!("unary minus is supported for numbers and float only"),
        },
        other => panic!("unknown or unsupported expression: {other:?}"),
    }
}

pub fn parse_dictionary<'a, T>(iter: &mut T) -> DictValue<'a>
where
    T: Iterator<Item = Token<'a>>,
{
    let mut vec: DictValue<'a> = vec![];

    match next_non_blank!(iter) {
        Token::ClosingBrace => (), // empty dict
        Token::Identifier(s) => parse_lua_dict(iter, &mut vec, ASTValue::String(s)),
        other => {
            let first_key = parse_value(iter, Some(other));
            parse_python_dict(iter, &mut vec, first_key);
        }
    }

    vec
}

pub fn parse_lua_dict<'a, T>(iter: &mut T, vec: &mut DictValue<'a>, first_key: ASTValue<'a>)
where
    T: Iterator<Item = Token<'a>>,
{
    expect_blank_prefixed!(iter, Token::Assignment, ());
    let first_val = parse_value(iter, None);
    vec.push((first_key, first_val));

    let mut expect_comma = true; // just got our pair, expect a comma

    loop {
        match next_non_blank!(iter) {
            Token::Comma => {
                if !expect_comma {
                    panic!("unexpected comma, expected a value");
                }
                expect_comma = false;
            }
            Token::Identifier(s) => {
                expect_blank_prefixed!(iter, Token::Assignment, ());
                vec.push((ASTValue::String(s), parse_value(iter, None)));
                expect_comma = true;
            }
            Token::ClosingBrace => break,
            other => panic!("unexpected {other:?}"),
        }
    }
}

pub fn parse_python_dict<'a, T>(iter: &mut T, vec: &mut DictValue<'a>, first_key: ASTValue<'a>)
where
    T: Iterator<Item = Token<'a>>,
{
    expect_blank_prefixed!(iter, Token::Colon, ());
    let first_val = parse_value(iter, None);
    vec.push((first_key, first_val));

    let mut expect_comma = true; // just got our pair, expect a comma

    loop {
        match next_non_blank!(iter) {
            Token::Comma => {
                if !expect_comma {
                    panic!("unexpected comma, expected a value");
                }
                expect_comma = false;
            }
            Token::ClosingBrace => break,
            other => {
                let key = parse_value(iter, Some(other));
                expect_blank_prefixed!(iter, Token::Colon, ());
                vec.push((key, parse_value(iter, None)));
                expect_comma = true;
            }
        }
    }
}

pub fn parse_enum<'a, T>(iter: &mut T) -> ASTEnum<'a>
where
    T: Iterator<Item = Token<'a>>,
{
    let identifier = match next_non_blank!(iter) {
        Token::Identifier(s) => {
            expect_blank_prefixed!(iter, Token::OpeningBrace, ());
            Some(s)
        }
        Token::OpeningBrace => None,
        other => panic!("unexpected {other:?}, expected identifier or opening brace"),
    };

    let mut variants = vec![];
    let mut expect_comma = false;

    loop {
        match next_non_blank!(iter) {
            Token::Comma => {
                if !expect_comma {
                    panic!("unexpected comma, expected a value");
                }
                expect_comma = false;
            }
            Token::Identifier(identifier) => {
                if expect_comma {
                    panic!("unexpected identifier, expected comma");
                }

                match next_non_blank!(iter) {
                    Token::Comma => variants.push(ASTEnumVariant {
                        identifier,
                        value: None,
                    }),
                    Token::Assignment => {
                        let value = Some(parse_value(iter, None).into_number().unwrap());
                        variants.push(ASTEnumVariant { identifier, value });
                        expect_comma = true;
                    }
                    Token::ClosingBrace => {
                        variants.push(ASTEnumVariant {
                            identifier,
                            value: None,
                        });
                        break;
                    }
                    other => {
                        panic!("unxpected {other:?}, expected comma, assignment or closing brace")
                    }
                }
            }
            Token::ClosingBrace => break,
            other => panic!("unexpected {other:?}"),
        }
    }

    ASTEnum {
        identifier,
        variants,
    }
}

pub fn parse_func<'a, T>(iter: &mut T) -> ASTFunction<'a>
where
    T: Iterator<Item = Token<'a>>,
{
    expect!(iter, Token::Blank(_), ());
    let identifier = expect_blank_prefixed!(iter, Token::Identifier(s), s);
    expect_blank_prefixed!(iter, Token::OpeningParenthesis, ());
    let mut parameters = vec![];

    loop {
        let mut expect_comma = true;
        let mut break_ = false;

        let (identifier, infer_type, typehint, default) = parse_idtydef!(
            iter,
            Token::Comma => { dbg!("got comma"); expect_comma = false; },
            Token::ClosingParenthesis => { break_ = true; dbg!("got end paren"); },
        );

        parameters.push(ASTFunctionParameter {
            identifier,
            infer_type,
            typehint,
            default,
        });

        if break_ {
            break;
        }

        if expect_comma {
            match next_non_blank!(iter) {
                Token::Comma => (),
                Token::ClosingParenthesis => break,
                other => panic!("expected comma or closing parenthesis, found {other:?}"),
            }
        }
    }

    expect_blank_prefixed!(iter, Token::Colon, ());

    ASTFunction {
        identifier,
        parameters,
        body: vec![],
    }
}

pub fn parse_func_body<'a, T>(iter: &mut T) -> CodeBlock<'a>
where
    T: Iterator<Item = Token<'a>>,
{
    vec![]
}
