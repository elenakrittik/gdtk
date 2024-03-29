use std::iter::Peekable;

use gdtk_ast::poor::{ASTBinaryOp, ASTUnaryOp, ASTValue, DictValue};
use gdtk_lexer::{Token, TokenKind};

use crate::{
    functions::parse_func,
    utils::{
        collect_args, collect_args_raw, expect_blank_prefixed, next_non_blank, peek_non_blank,
    },
};

pub fn parse_value<'a, T>(iter: &mut Peekable<T>, mut token: Option<Token<'a>>) -> ASTValue<'a>
where
    T: Iterator<Item = Token<'a>>,
{
    unimplemented!()
}

pub fn parse_value_with_ops<'a, T>(iter: &mut Peekable<T>) -> ASTValue<'a>
where
    T: Iterator<Item = Token<'a>>,
{
    #[derive(Debug)]
    enum ValueOrOp<'b> {
        Value(ASTValue<'b>),
        Op(ASTBinaryOp),
    }

    let mut values_and_ops: Vec<ValueOrOp<'_>> = vec![];

    loop {
        let value = parse_value_with_unary_ops(iter);
        values_and_ops.push(ValueOrOp::Value(value));

        if let Some(op) = match peek_non_blank(iter) {
            Some(Token { kind: TokenKind::Plus, .. }) => Some(ASTBinaryOp::Plus),
            Some(Token { kind: TokenKind::Minus, .. }) => Some(ASTBinaryOp::Minus),
            Some(Token { kind: TokenKind::Greater, .. }) => Some(ASTBinaryOp::Greater),
            Some(Token { kind: TokenKind::GreaterOrEqual, .. }) => Some(ASTBinaryOp::GreaterOrEqual),
            Some(Token { kind: TokenKind::Less, .. }) => Some(ASTBinaryOp::Less),
            Some(Token { kind: TokenKind::LessOrEqual, .. }) => Some(ASTBinaryOp::LessOrEqual),
            Some(Token { kind: TokenKind::Period, .. }) => Some(ASTBinaryOp::PropertyAccess),
            Some(Token { kind: TokenKind::Multiply, .. }) => Some(ASTBinaryOp::Multiply),
            Some(Token { kind: TokenKind::Divide, .. }) => Some(ASTBinaryOp::Divide),
            Some(Token { kind: TokenKind::Equal, .. }) => Some(ASTBinaryOp::Equal),
            Some(Token { kind: TokenKind::NotEqual, .. }) => Some(ASTBinaryOp::NotEqual),
            Some(Token { kind: TokenKind::And | TokenKind::SymbolizedAnd, .. }) => Some(ASTBinaryOp::And),
            Some(Token { kind: TokenKind::Or | TokenKind::SymbolizedOr, .. }) => Some(ASTBinaryOp::Or),
            Some(Token { kind: TokenKind::Not | TokenKind::SymbolizedNot, .. }) => Some(ASTBinaryOp::Not),
            Some(Token { kind: TokenKind::BitwiseAnd, .. }) => Some(ASTBinaryOp::BitwiseAnd),
            Some(Token { kind: TokenKind::BitwiseOr, .. }) => Some(ASTBinaryOp::BitwiseOr),
            Some(Token { kind: TokenKind::BitwiseNot, .. }) => Some(ASTBinaryOp::BitwiseNot),
            Some(Token { kind: TokenKind::BitwiseXor, .. }) => Some(ASTBinaryOp::BitwiseXor),
            Some(Token { kind: TokenKind::BitwiseShiftLeft, .. }) => Some(ASTBinaryOp::BitwiseShiftLeft),
            Some(Token { kind: TokenKind::BitwiseShiftRight, .. }) => Some(ASTBinaryOp::BitwiseShiftRight),
            Some(Token { kind: TokenKind::Power, .. }) => Some(ASTBinaryOp::Power),
            Some(Token { kind: TokenKind::Remainder, .. }) => Some(ASTBinaryOp::Remainder),
            Some(Token { kind: TokenKind::As, .. }) => Some(ASTBinaryOp::TypeCast),
            Some(Token { kind: TokenKind::Is, .. }) => Some(ASTBinaryOp::TypeCheck),
            Some(Token { kind: TokenKind::In, .. }) => Some(ASTBinaryOp::Contains),
            Some(Token { kind: TokenKind::Range, .. }) => Some(ASTBinaryOp::Range),
            _ => None,
        } {
            iter.next();

            values_and_ops.push(ValueOrOp::Op(op));
        } else {
            break;
        }
    }

    todo!()
}

/// Parses a value taking into account possible prefix and postfix OPs
pub fn parse_value_with_unary_ops<'a, T>(iter: &mut Peekable<T>) -> ASTValue<'a>
where
    T: Iterator<Item = Token<'a>>,
{
    let mut prefix_ops = vec![];

    while let Some(op) = match peek_non_blank(iter) {
            Some(Token { kind: TokenKind::Plus, .. }) => Some(ASTUnaryOp::Plus),
            Some(Token { kind: TokenKind::Minus, .. }) => Some(ASTUnaryOp::Minus),
            Some(Token { kind: TokenKind::Await, .. }) => Some(ASTUnaryOp::Await),
            None => panic!("expected expression"),
            _ => None,
    } {
        iter.next();
        prefix_ops.push(op);
    }

    let mut value = parse_value_without_ops(iter);

    // Calls have higher precedence, i.e. `-get_num()` should be parsed as `-(get_num())`
    if let Some(Token { kind: TokenKind::OpeningParenthesis, .. }) = peek_non_blank(iter) {
        value = ASTValue::Call(Box::new(value), collect_args!(iter, TokenKind::OpeningParenthesis, TokenKind::ClosingParenthesis));
    }

    for op in prefix_ops {
        value = ASTValue::UnaryExpr(op, Box::new(value));
    }

    value
}

/// Parses a "clean" value, without checking for possible prefix or postfix OPs
pub fn parse_value_without_ops<'a, T>(iter: &mut Peekable<T>) -> ASTValue<'a>
where
    T: Iterator<Item = Token<'a>>,
{
    let token = next_non_blank!(iter);

    match token.kind {
        TokenKind::Identifier(s) => ASTValue::Identifier(s),
        TokenKind::Integer(i) => ASTValue::Number(i),
        TokenKind::BinaryInteger(i) => ASTValue::Number(i as i64),
        TokenKind::HexInteger(i) => ASTValue::Number(i as i64),
        TokenKind::ScientificFloat(f) => ASTValue::Float(f),
        TokenKind::Float(f) => ASTValue::Float(f),
        TokenKind::String(s) => ASTValue::String(s),
        TokenKind::StringName(s) => ASTValue::StringName(s),
        TokenKind::Node(s) => ASTValue::Node(s),
        TokenKind::UniqueNode(s) => ASTValue::UniqueNode(s),
        TokenKind::NodePath(s) => ASTValue::NodePath(s),
        TokenKind::Boolean(b) => ASTValue::Boolean(b),
        TokenKind::OpeningBracket => ASTValue::Array(collect_args_raw!(iter, TokenKind::ClosingBracket)),
        TokenKind::OpeningBrace => ASTValue::Dictionary(parse_dictionary(iter)),
        TokenKind::Comment(c) => ASTValue::Comment(c),
        TokenKind::Func => ASTValue::Lambda(parse_func(iter, true)),
        _ => panic!("unknown or unsupported expression: {token:?}"),
    }
}

pub fn parse_dictionary<'a, T>(iter: &mut Peekable<T>) -> DictValue<'a>
where
T: Iterator<Item = Token<'a>>,
{
    let mut vec: DictValue<'a> = vec![];
    
    match next_non_blank!(iter) {
        Token {
            kind: TokenKind::ClosingBrace,
            ..
        } => (), // empty dict
        Token {
            kind: TokenKind::Identifier(s),
            ..
        } => parse_lua_dict(iter, &mut vec, ASTValue::String(s)),
        other => {
            let first_key = parse_value(iter, Some(other));
            parse_python_dict(iter, &mut vec, first_key);
        }
    }

    vec
}

pub fn parse_lua_dict<'a, T>(
    iter: &mut Peekable<T>,
    vec: &mut DictValue<'a>,
    first_key: ASTValue<'a>,
) where
    T: Iterator<Item = Token<'a>>,
{
    expect_blank_prefixed!(iter, TokenKind::Assignment, ());
    let first_val = parse_value(iter, None);
    vec.push((first_key, first_val));

    let mut expect_comma = true; // just got our pair, expect a comma

    loop {
        match next_non_blank!(iter) {
            Token {
                kind: TokenKind::Comma,
                ..
            } => {
                if !expect_comma {
                    panic!("unexpected comma, expected a value");
                }
                expect_comma = false;
            }
            Token {
                kind: TokenKind::Identifier(s),
                ..
            } => {
                expect_blank_prefixed!(iter, TokenKind::Assignment, ());
                vec.push((ASTValue::String(s), parse_value(iter, None)));
                expect_comma = true;
            }
            Token {
                kind: TokenKind::ClosingBrace,
                ..
            } => break,
            other => panic!("unexpected {other:?}"),
        }
    }
}

pub fn parse_python_dict<'a, T>(
    iter: &mut Peekable<T>,
    vec: &mut DictValue<'a>,
    first_key: ASTValue<'a>,
) where
    T: Iterator<Item = Token<'a>>,
{
    expect_blank_prefixed!(iter, TokenKind::Colon, ());
    let first_val = parse_value(iter, None);
    vec.push((first_key, first_val));

    let mut expect_comma = true; // just got our pair, expect a comma

    loop {
        match next_non_blank!(iter) {
            Token {
                kind: TokenKind::Comma,
                ..
            } => {
                if !expect_comma {
                    panic!("unexpected comma, expected a value");
                }
                expect_comma = false;
            }
            Token {
                kind: TokenKind::ClosingBrace,
                ..
            } => break,
            other => {
                let key = parse_value(iter, Some(other));
                expect_blank_prefixed!(iter, TokenKind::Colon, ());
                vec.push((key, parse_value(iter, None)));
                expect_comma = true;
            }
        }
    }
}
