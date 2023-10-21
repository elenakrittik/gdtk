// #![feature(type_alias_impl_trait)]

// use gdtk_ast::poor::ASTClass;
use gdtk_lexer::{token::Token, LexOutput};

// use crate::error::Error;

pub mod error;

// type TokenIter<'a> = impl Iterator<Item = Token<'a>>;

pub fn parse_file<'a>(lexed: LexOutput<'a>) {
    // -> Result<ASTClass, Error> {
    let (tokens, _diags) = lexed;
    dbg!(&tokens);

    let mut iter = tokens.into_iter().map(|v| v.0);

    match iter.next().unwrap_or(Token::Newline) {
        Token::Identifier(_) => todo!(),
        Token::Integer(_) => todo!(),
        Token::BinaryInteger(_) => todo!(),
        Token::HexInteger(_) => todo!(),
        Token::ScientificFloat(_) => todo!(),
        Token::Float(_) => todo!(),
        Token::String(_) => todo!(),
        Token::StringName(_) => todo!(),
        Token::Node(_) => todo!(),
        Token::UniqueNode(_) => todo!(),
        Token::NodePath(_) => todo!(),
        Token::Boolean(_) => todo!(),
        Token::Null => todo!(),
        Token::Less => todo!(),
        Token::LessEqual => todo!(),
        Token::Greater => todo!(),
        Token::GreaterEqual => todo!(),
        Token::Equal => todo!(),
        Token::NotEqual => todo!(),
        Token::And => todo!(),
        Token::Or => todo!(),
        Token::Not => todo!(),
        Token::SymbolizedAnd => todo!(),
        Token::SymbolizedOr => todo!(),
        Token::SymbolizedNot => todo!(),
        Token::BitwiseAnd => todo!(),
        Token::BitwiseOr => todo!(),
        Token::BitwiseNot => todo!(),
        Token::BitwiseXor => todo!(),
        Token::BitwiseShiftLeft => todo!(),
        Token::BitwiseShiftRight => todo!(),
        Token::Plus => todo!(),
        Token::Minus => todo!(),
        Token::Multiply => todo!(),
        Token::Power => todo!(),
        Token::Divide => todo!(),
        Token::Remainder => todo!(),
        Token::Assignment => todo!(),
        Token::PlusAssignment => todo!(),
        Token::MinusAssignment => todo!(),
        Token::MultiplyAssignment => todo!(),
        Token::PowerAssignment => todo!(),
        Token::DivideAssignment => todo!(),
        Token::RemainderAssignment => todo!(),
        Token::BitwiseAndAssignment => todo!(),
        Token::BitwiseOrAssignment => todo!(),
        Token::BitwiseNotAssignment => todo!(),
        Token::BitwiseXorAssignment => todo!(),
        Token::BitwiseShiftLeftAssignment => todo!(),
        Token::BitwiseShiftRightAssignment => todo!(),
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
        Token::ClassName => parse_classname(iter),
        Token::Const => todo!(),   // parse_const(iter),
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
        Token::Blank(_) => (), // todo: should this be an error?
        Token::Comment(_) => (),
        Token::Namespace => todo!(),
        Token::Trait => todo!(),
        Token::Yield => todo!(),
    }

    ()
}

pub fn parse_classname<'a>(mut iter: impl Iterator<Item = Token<'a>>) {
    let blank = match iter.next() {
        Some(t) => t,
        None => panic!("expected at least one space after class_name"),
    };

    if !matches!(blank, Token::Blank(_)) {
        panic!("expected at least one space after class_name")
    }

    let mut ident = None;

    for token in iter {
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

    dbg!(ident);
}
