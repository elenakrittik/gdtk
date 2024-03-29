use std::iter::Peekable;

use gdtk_ast::poor::{ASTValue, ASTBinaryOp, ASTUnaryOp};
use gdtk_lexer::{Token, TokenKind};

use crate::{functions::parse_func, utils::{collect_args, collect_args_raw, next_non_blank, peek_non_blank}, values::parse_dictionary};

pub fn parse_expression<'a, T>(iter: &mut Peekable<T>) -> ASTValue<'a>
where
    T: Iterator<Item = Token<'a>>,
{
    let initial_value = parse_expr_with_ops(iter);
    
    let mut values_and_ops= vec![];

    loop {
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

            let value = parse_expr_with_ops(iter);
            values_and_ops.push((op, value));
        } else {
            break;
        }
    }

    let expr = prec::Expression::new(initial_value, values_and_ops);

    climber().process(&expr, &()).unwrap()
}

/// Parses a value taking into account possible prefix and postfix OPs
pub fn parse_expr_with_ops<'a, T>(iter: &mut Peekable<T>) -> ASTValue<'a>
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

    let mut value = parse_expr_without_ops(iter);

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
pub fn parse_expr_without_ops<'a, T>(iter: &mut Peekable<T>) -> ASTValue<'a>
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

fn handler<'a>(lhs: ASTValue<'a>, op: ASTBinaryOp, rhs: ASTValue<'a>, _ctx: &()) -> Result<ASTValue<'a>, ()> {
    Ok(ASTValue::BinaryExpr(Box::new(lhs), op, Box::new(rhs)))
}

fn climber<'a>() -> prec::Climber<ASTBinaryOp, ASTValue<'a>, ASTValue<'a>, ()> {
    // i have no idea what i did here and am just praying that it'll work
    // if it does not, its easy to fix anyway

    // if someone can read the official sources, corrections are welcome
    // https://github.com/godotengine/godot/blob/master/modules/gdscript/gdscript_parser.cpp#L3847
    prec::Climber::new(
        vec![
            prec::Rule::new(ASTBinaryOp::Plus, prec::Assoc::Left)
            | prec::Rule::new(ASTBinaryOp::Minus, prec::Assoc::Left),

            prec::Rule::new(ASTBinaryOp::Multiply, prec::Assoc::Left)
            | prec::Rule::new(ASTBinaryOp::Divide, prec::Assoc::Left),

            prec::Rule::new(ASTBinaryOp::Remainder, prec::Assoc::Left),
            prec::Rule::new(ASTBinaryOp::Power, prec::Assoc::Right),
            prec::Rule::new(ASTBinaryOp::Range, prec::Assoc::Right),
            prec::Rule::new(ASTBinaryOp::BitwiseOr, prec::Assoc::Left)

            | prec::Rule::new(ASTBinaryOp::BitwiseXor, prec::Assoc::Left)
            | prec::Rule::new(ASTBinaryOp::BitwiseAnd, prec::Assoc::Left),

            prec::Rule::new(ASTBinaryOp::BitwiseShiftLeft, prec::Assoc::Left)
            | prec::Rule::new(ASTBinaryOp::BitwiseShiftRight, prec::Assoc::Left),

            prec::Rule::new(ASTBinaryOp::BitwiseNot, prec::Assoc::Right),

            prec::Rule::new(ASTBinaryOp::TypeCast, prec::Assoc::Right)
            | prec::Rule::new(ASTBinaryOp::TypeCheck, prec::Assoc::Right),

            prec::Rule::new(ASTBinaryOp::Contains, prec::Assoc::Right),
            prec::Rule::new(ASTBinaryOp::PropertyAccess, prec::Assoc::Left),

            prec::Rule::new(ASTBinaryOp::And, prec::Assoc::Left)
            | prec::Rule::new(ASTBinaryOp::Or, prec::Assoc::Left),

            prec::Rule::new(ASTBinaryOp::Less, prec::Assoc::Left)
            | prec::Rule::new(ASTBinaryOp::LessOrEqual, prec::Assoc::Left)
            | prec::Rule::new(ASTBinaryOp::Greater, prec::Assoc::Left)
            | prec::Rule::new(ASTBinaryOp::GreaterOrEqual, prec::Assoc::Left)
            | prec::Rule::new(ASTBinaryOp::Equal, prec::Assoc::Left)
            | prec::Rule::new(ASTBinaryOp::NotEqual, prec::Assoc::Left),

            prec::Rule::new(ASTBinaryOp::Not, prec::Assoc::Right),
        ],
        handler,
    )
}
