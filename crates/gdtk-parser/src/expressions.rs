use gdtk_ast::{ASTBinaryOp, ASTExpr, ASTPostfixOp, ASTPrefixOp};
use gdtk_lexer::{Token, TokenKind};
use pratt::{Affix, Associativity, PrattParser, Precedence};

use crate::{
    utils::{advance_and_parse, delemited_by, expect},
    values::{parse_array, parse_dictionary, parse_lambda},
    Parser,
};

/// Parse an expression.
pub fn parse_expr<'a>(parser: &mut Parser<impl Iterator<Item = Token<'a>>>) -> ASTExpr<'a> {
    ExprParser
        .parse(parse_expr_impl(parser).into_iter())
        .unwrap()
}

fn parse_expr_impl<'a>(parser: &mut Parser<impl Iterator<Item = Token<'a>>>) -> Vec<ExprIR<'a>> {
    let mut result = parse_expr_with_ops(parser);

    while let Some(op) = match parser.peek().map(|t| &t.kind) {
        Some(TokenKind::Plus) => Some(ASTBinaryOp::Add),
        Some(TokenKind::Minus) => Some(ASTBinaryOp::Subtract),
        Some(TokenKind::GreaterThan) => Some(ASTBinaryOp::Greater),
        Some(TokenKind::GreaterThanOrEqual) => Some(ASTBinaryOp::GreaterOrEqual),
        Some(TokenKind::LessThan) => Some(ASTBinaryOp::LessThan),
        Some(TokenKind::LessThanOrEqual) => Some(ASTBinaryOp::LessOrEqual),
        Some(TokenKind::Period) => Some(ASTBinaryOp::PropertyAccess),
        Some(TokenKind::Multiply) => Some(ASTBinaryOp::Multiply),
        Some(TokenKind::Divide) => Some(ASTBinaryOp::Divide),
        Some(TokenKind::Equal) => Some(ASTBinaryOp::Equals),
        Some(TokenKind::NotEqual) => Some(ASTBinaryOp::NotEqual),
        Some(TokenKind::And | TokenKind::SymbolizedAnd) => Some(ASTBinaryOp::And),
        Some(TokenKind::Or | TokenKind::SymbolizedOr) => Some(ASTBinaryOp::Or),
        Some(TokenKind::BitwiseAnd) => Some(ASTBinaryOp::BitwiseAnd),
        Some(TokenKind::BitwiseOr) => Some(ASTBinaryOp::BitwiseOr),
        Some(TokenKind::BitwiseXor) => Some(ASTBinaryOp::BitwiseXor),
        Some(TokenKind::BitwiseShiftLeft) => Some(ASTBinaryOp::BitwiseShiftLeft),
        Some(TokenKind::BitwiseShiftRight) => Some(ASTBinaryOp::BitwiseShiftRight),
        Some(TokenKind::Power) => Some(ASTBinaryOp::Power),
        Some(TokenKind::Remainder) => Some(ASTBinaryOp::Remainder),
        Some(TokenKind::As) => Some(ASTBinaryOp::TypeCast),
        Some(TokenKind::Is) => Some(ASTBinaryOp::TypeCheck),
        Some(TokenKind::In) => Some(ASTBinaryOp::Contains),
        Some(TokenKind::NotIn) => Some(ASTBinaryOp::NotContains),
        Some(TokenKind::Range) => Some(ASTBinaryOp::Range),
        Some(TokenKind::Assignment) => Some(ASTBinaryOp::Assignment),
        Some(TokenKind::PlusAssignment) => Some(ASTBinaryOp::PlusAssignment),
        Some(TokenKind::MinusAssignment) => Some(ASTBinaryOp::MinusAssignment),
        Some(TokenKind::MultiplyAssignment) => Some(ASTBinaryOp::MultiplyAssignment),
        Some(TokenKind::PowerAssignment) => Some(ASTBinaryOp::PowerAssignment),
        Some(TokenKind::DivideAssignment) => Some(ASTBinaryOp::DivideAssignment),
        Some(TokenKind::RemainderAssignment) => Some(ASTBinaryOp::RemainderAssignment),
        Some(TokenKind::BitwiseAndAssignment) => Some(ASTBinaryOp::BitwiseAndAssignment),
        Some(TokenKind::BitwiseOrAssignment) => Some(ASTBinaryOp::BitwiseNotAssignment),
        Some(TokenKind::BitwiseNotAssignment) => Some(ASTBinaryOp::BitwiseNotAssignment),
        Some(TokenKind::BitwiseXorAssignment) => Some(ASTBinaryOp::BitwiseXorAssignment),
        Some(TokenKind::BitwiseShiftLeftAssignment) => {
            Some(ASTBinaryOp::BitwiseShiftLeftAssignment)
        }
        Some(TokenKind::BitwiseShiftRightAssignment) => {
            Some(ASTBinaryOp::BitwiseShiftRightAssignment)
        }
        Some(TokenKind::If) => Some(ASTBinaryOp::TernaryIfElsePlaceholder),
        _ => None,
    } {
        parser.next();

        match op {
            ASTBinaryOp::TernaryIfElsePlaceholder => {
                let op = ASTBinaryOp::TernaryIfElse(Box::new(parse_expr(parser)));
                result.push(ExprIR::Binary(op));
                expect!(parser, TokenKind::Else);
                result.push(ExprIR::Primary(parse_expr(parser)));
            }
            other => {
                result.push(ExprIR::Binary(other));
                result.extend(parse_expr_with_ops(parser));
            }
        }
    }

    result
}

/// Parses a value taking into account possible prefix and postfix OPs
fn parse_expr_with_ops<'a>(
    parser: &mut Parser<impl Iterator<Item = Token<'a>>>,
) -> Vec<ExprIR<'a>> {
    let mut result = vec![];

    while let Some(op) = match parser.peek().map(|t| &t.kind) {
        Some(TokenKind::Plus) => Some(ASTPrefixOp::Identity),
        Some(TokenKind::Minus) => Some(ASTPrefixOp::Negation),
        Some(TokenKind::Await) => Some(ASTPrefixOp::Await),
        Some(TokenKind::BitwiseNot) => Some(ASTPrefixOp::BitwiseNot),
        Some(TokenKind::Not | TokenKind::SymbolizedNot) => Some(ASTPrefixOp::Not),
        None => panic!("expected expression"),
        _ => None,
    } {
        parser.next();
        result.push(ExprIR::Prefix(op));
    }

    result.push(parse_expr_without_ops(parser));

    loop {
        if !matches!(
            parser.peek(),
            Some(Token {
                kind: TokenKind::OpeningParenthesis | TokenKind::OpeningBracket,
                ..
            })
        ) {
            break;
        }

        match parser.next().unwrap().kind {
            TokenKind::OpeningParenthesis => {
                let values = parser.with_parens_ctx(true, |parser| {
                    delemited_by(
                        parser,
                        TokenKind::Comma,
                        &[TokenKind::ClosingParenthesis],
                        parse_expr,
                    )
                });

                expect!(parser, TokenKind::ClosingParenthesis);

                result.push(ExprIR::Postfix(ASTPostfixOp::Call(values)));
            }
            TokenKind::OpeningBracket => {
                let values = parser.with_parens_ctx(true, |parser| {
                    delemited_by(
                        parser,
                        TokenKind::Comma,
                        &[TokenKind::ClosingBracket],
                        parse_expr,
                    )
                });

                expect!(parser, TokenKind::ClosingBracket);

                result.push(ExprIR::Postfix(ASTPostfixOp::Subscript(values)));
            }
            _ => unreachable!(),
        }
    }

    result
}

/// Parses a "clean" value, without checking for possible prefix or postfix OPs
fn parse_expr_without_ops<'a>(parser: &mut Parser<impl Iterator<Item = Token<'a>>>) -> ExprIR<'a> {
    let value = match &parser.peek().expect("unexpected EOF").kind {
        TokenKind::Identifier(_) => {
            ASTExpr::Identifier(parser.next().unwrap().kind.into_identifier().unwrap())
        }
        TokenKind::Integer(_) => {
            ASTExpr::Number(parser.next().unwrap().kind.into_integer().unwrap())
        }
        TokenKind::BinaryInteger(_) => {
            ASTExpr::Number(parser.next().unwrap().kind.into_binary_integer().unwrap())
        }
        TokenKind::HexInteger(_) => {
            ASTExpr::Number(parser.next().unwrap().kind.into_hex_integer().unwrap())
        }
        TokenKind::Float(_) => ASTExpr::Float(parser.next().unwrap().kind.into_float().unwrap()),
        TokenKind::ScientificFloat(_) => {
            ASTExpr::Float(parser.next().unwrap().kind.into_scientific_float().unwrap())
        }
        TokenKind::String(_) => ASTExpr::String(parser.next().unwrap().kind.into_string().unwrap()),
        TokenKind::StringName(_) => {
            ASTExpr::StringName(parser.next().unwrap().kind.into_string_name().unwrap())
        }
        TokenKind::Node(_) => ASTExpr::Node(parser.next().unwrap().kind.into_node().unwrap()),
        TokenKind::UniqueNode(_) => {
            ASTExpr::UniqueNode(parser.next().unwrap().kind.into_unique_node().unwrap())
        }
        TokenKind::NodePath(_) => {
            ASTExpr::NodePath(parser.next().unwrap().kind.into_node_path().unwrap())
        }
        TokenKind::Boolean(_) => {
            ASTExpr::Boolean(parser.next().unwrap().kind.into_boolean().unwrap())
        }
        TokenKind::Func => ASTExpr::Lambda(parse_lambda(parser)),
        TokenKind::OpeningBracket => ASTExpr::Array(parse_array(parser)),
        TokenKind::OpeningBrace => ASTExpr::Dictionary(parse_dictionary(parser)),
        TokenKind::OpeningParenthesis => {
            parser.next();

            let values = parser.with_parens_ctx(true, |parser| {
                delemited_by(
                    parser,
                    TokenKind::Comma,
                    &[TokenKind::ClosingParenthesis],
                    parse_expr,
                )
            });

            expect!(parser, TokenKind::ClosingParenthesis);

            return ExprIR::Group(values);
        }
        TokenKind::Null => advance_and_parse(parser, |_| ASTExpr::Null),
        _ => panic!("unknown or unsupported expression: {:#?}", parser.peek()),
    };

    ExprIR::Primary(value)
}

#[derive(Debug, enum_as_inner::EnumAsInner)]
enum ExprIR<'a> {
    Prefix(ASTPrefixOp),
    Postfix(ASTPostfixOp<'a>),
    Binary(ASTBinaryOp<'a>),
    Group(Vec<ASTExpr<'a>>),
    Primary(ASTExpr<'a>),
}

struct ExprParser;

impl<'a, I> pratt::PrattParser<I> for ExprParser
where
    I: Iterator<Item = ExprIR<'a>>,
{
    type Error = pratt::NoError;
    type Input = ExprIR<'a>;
    type Output = ASTExpr<'a>;

    #[rustfmt::skip]
    fn query(&mut self, input: &Self::Input) -> Result<Affix, Self::Error> {
        Ok(match input {
            ExprIR::Primary(_) => Affix::Nilfix,
            ExprIR::Group(_) => Affix::Nilfix,

            ExprIR::Postfix(ASTPostfixOp::Subscript(_)) => Affix::Postfix(Precedence(23)),

            ExprIR::Binary(ASTBinaryOp::PropertyAccess) => Affix::Infix(Precedence(22), Associativity::Left),

            ExprIR::Postfix(ASTPostfixOp::Call(_)) => Affix::Postfix(Precedence(21)),

            ExprIR::Prefix(ASTPrefixOp::Await) => Affix::Prefix(Precedence(20)),

            ExprIR::Binary(ASTBinaryOp::TypeCheck) => Affix::Infix(Precedence(19), Associativity::Left),
            ExprIR::Binary(ASTBinaryOp::Power) => Affix::Infix(Precedence(18), Associativity::Left),

            ExprIR::Prefix(ASTPrefixOp::BitwiseNot) => Affix::Prefix(Precedence(17)),

            ExprIR::Prefix(ASTPrefixOp::Identity) => Affix::Prefix(Precedence(16)),
            ExprIR::Prefix(ASTPrefixOp::Negation) => Affix::Prefix(Precedence(16)),

            ExprIR::Binary(ASTBinaryOp::Multiply) => Affix::Infix(Precedence(15), Associativity::Left),
            ExprIR::Binary(ASTBinaryOp::Divide) => Affix::Infix(Precedence(15), Associativity::Left),
            ExprIR::Binary(ASTBinaryOp::Remainder) => Affix::Infix(Precedence(15), Associativity::Left),

            ExprIR::Binary(ASTBinaryOp::Add) => Affix::Infix(Precedence(14), Associativity::Left),
            ExprIR::Binary(ASTBinaryOp::Subtract) => Affix::Infix(Precedence(14), Associativity::Left),

            ExprIR::Binary(ASTBinaryOp::BitwiseShiftLeft) => Affix::Infix(Precedence(13), Associativity::Left),
            ExprIR::Binary(ASTBinaryOp::BitwiseShiftRight) => Affix::Infix(Precedence(13), Associativity::Left),

            ExprIR::Binary(ASTBinaryOp::BitwiseAnd) => Affix::Infix(Precedence(12), Associativity::Left),

            ExprIR::Binary(ASTBinaryOp::BitwiseXor) => Affix::Infix(Precedence(11), Associativity::Left),

            ExprIR::Binary(ASTBinaryOp::BitwiseOr) => Affix::Infix(Precedence(10), Associativity::Left),

            ExprIR::Binary(ASTBinaryOp::Equals) => Affix::Infix(Precedence(9), Associativity::Left),
            ExprIR::Binary(ASTBinaryOp::NotEqual) => Affix::Infix(Precedence(9), Associativity::Left),
            ExprIR::Binary(ASTBinaryOp::LessThan) => Affix::Infix(Precedence(9), Associativity::Left),
            ExprIR::Binary(ASTBinaryOp::LessOrEqual) => Affix::Infix(Precedence(9), Associativity::Left),
            ExprIR::Binary(ASTBinaryOp::Greater) => Affix::Infix(Precedence(9), Associativity::Left),
            ExprIR::Binary(ASTBinaryOp::GreaterOrEqual) => Affix::Infix(Precedence(9), Associativity::Left),

            ExprIR::Binary(ASTBinaryOp::Contains) => Affix::Infix(Precedence(8), Associativity::Left),
            ExprIR::Binary(ASTBinaryOp::NotContains) => Affix::Infix(Precedence(8), Associativity::Left),

            ExprIR::Prefix(ASTPrefixOp::Not) => Affix::Prefix(Precedence(7)),

            ExprIR::Binary(ASTBinaryOp::And) => Affix::Infix(Precedence(6), Associativity::Left),

            ExprIR::Binary(ASTBinaryOp::Or) => Affix::Infix(Precedence(5), Associativity::Left),

            ExprIR::Binary(ASTBinaryOp::TernaryIfElse(_)) => Affix::Infix(Precedence(4), Associativity::Right),
            ExprIR::Binary(ASTBinaryOp::TernaryIfElsePlaceholder) => unreachable!(),

            ExprIR::Binary(ASTBinaryOp::Range) => Affix::Infix(Precedence(3), Associativity::Left),

            ExprIR::Binary(ASTBinaryOp::TypeCast) => Affix::Infix(Precedence(2), Associativity::Left),

            ExprIR::Binary(ASTBinaryOp::Assignment) => Affix::Infix(Precedence(1), Associativity::Left),
            ExprIR::Binary(ASTBinaryOp::PlusAssignment) => Affix::Infix(Precedence(1), Associativity::Left),
            ExprIR::Binary(ASTBinaryOp::MinusAssignment) => Affix::Infix(Precedence(1), Associativity::Left),
            ExprIR::Binary(ASTBinaryOp::MultiplyAssignment) => Affix::Infix(Precedence(1), Associativity::Left),
            ExprIR::Binary(ASTBinaryOp::DivideAssignment) => Affix::Infix(Precedence(1), Associativity::Left),
            ExprIR::Binary(ASTBinaryOp::PowerAssignment) => Affix::Infix(Precedence(1), Associativity::Left),
            ExprIR::Binary(ASTBinaryOp::RemainderAssignment) => Affix::Infix(Precedence(1), Associativity::Left),
            ExprIR::Binary(ASTBinaryOp::BitwiseAndAssignment) => Affix::Infix(Precedence(1), Associativity::Left),
            ExprIR::Binary(ASTBinaryOp::BitwiseOrAssignment) => Affix::Infix(Precedence(1), Associativity::Left),
            ExprIR::Binary(ASTBinaryOp::BitwiseXorAssignment) => Affix::Infix(Precedence(1), Associativity::Left),
            ExprIR::Binary(ASTBinaryOp::BitwiseNotAssignment) => Affix::Infix(Precedence(1), Associativity::Left),
            ExprIR::Binary(ASTBinaryOp::BitwiseShiftLeftAssignment) => Affix::Infix(Precedence(1), Associativity::Left),
            ExprIR::Binary(ASTBinaryOp::BitwiseShiftRightAssignment) => Affix::Infix(Precedence(1), Associativity::Left),
        })
    }

    fn primary(&mut self, input: Self::Input) -> Result<Self::Output, Self::Error> {
        Ok(match input {
            ExprIR::Primary(val) => val,
            ExprIR::Group(vals) => ASTExpr::Group(vals),
            _ => unreachable!(),
        })
    }

    fn infix(
        &mut self,
        lhs: Self::Output,
        op: Self::Input,
        rhs: Self::Output,
    ) -> Result<Self::Output, Self::Error> {
        Ok(ASTExpr::BinaryExpr(
            Box::new(lhs),
            op.into_binary().unwrap(),
            Box::new(rhs),
        ))
    }

    fn prefix(&mut self, op: Self::Input, rhs: Self::Output) -> Result<Self::Output, Self::Error> {
        Ok(ASTExpr::PrefixExpr(
            op.into_prefix().unwrap(),
            Box::new(rhs),
        ))
    }

    fn postfix(&mut self, lhs: Self::Output, op: Self::Input) -> Result<Self::Output, Self::Error> {
        Ok(ASTExpr::PostfixExpr(
            Box::new(lhs),
            op.into_postfix().unwrap(),
        ))
    }
}

#[cfg(test)]
mod tests {
    use gdtk_ast::*;

    use crate::expressions::parse_expr;
    use crate::test_utils::create_parser;

    #[test]
    fn test_literals() {
        let inputs = [
            "ident",
            "0123556789",
            "0x123abc",
            "0.123456789",
            "1.0e2",
            "\"string\"",
            "&\"string\"",
            "$\"string\"",
            "%\"string\"",
            "^\"string\"",
            "true",
            "false",
            "null",
        ];
        let outputs = [
            ASTExpr::Identifier("ident"),
            ASTExpr::Number(123556789),
            ASTExpr::Number(0x123abc),
            ASTExpr::Float(0.123456789),
            ASTExpr::Float(1.0e2),
            ASTExpr::String("string"),
            ASTExpr::StringName("string"),
            ASTExpr::Node("string"),
            ASTExpr::UniqueNode("string"),
            ASTExpr::NodePath("string"),
            ASTExpr::Boolean(true),
            ASTExpr::Boolean(false),
            ASTExpr::Null,
        ];

        for (input, output) in inputs.into_iter().zip(outputs) {
            let mut parser = create_parser(input);

            assert_eq!(parse_expr(&mut parser), output);
        }
    }

    #[test]
    fn test_parse_prefix_exprs() {
        let inputs = ["await 1", "+1", "-1", "not 1", "!1", "~1"];
        let outputs = [
            ASTExpr::PrefixExpr(ASTPrefixOp::Await, Box::new(ASTExpr::Number(1))),
            ASTExpr::PrefixExpr(ASTPrefixOp::Identity, Box::new(ASTExpr::Number(1))),
            ASTExpr::PrefixExpr(ASTPrefixOp::Negation, Box::new(ASTExpr::Number(1))),
            ASTExpr::PrefixExpr(ASTPrefixOp::Not, Box::new(ASTExpr::Number(1))),
            ASTExpr::PrefixExpr(ASTPrefixOp::Not, Box::new(ASTExpr::Number(1))),
            ASTExpr::PrefixExpr(ASTPrefixOp::BitwiseNot, Box::new(ASTExpr::Number(1))),
        ];

        for (input, output) in inputs.into_iter().zip(outputs) {
            let mut parser = create_parser(input);

            assert_eq!(parse_expr(&mut parser), output);
        }
    }

    #[test]
    fn test_parse_postfix_exprs() {
        let inputs = ["1(2, 3)", "[1, 2, 3][0]", "1.1[0]", "{'foo': 'bar'}['foo']"];
        let outputs = [
            ASTExpr::PostfixExpr(
                Box::new(ASTExpr::Number(1)),
                ASTPostfixOp::Call(vec![ASTExpr::Number(2), ASTExpr::Number(3)]),
            ),
            ASTExpr::PostfixExpr(
                Box::new(ASTExpr::Array(vec![
                    ASTExpr::Number(1),
                    ASTExpr::Number(2),
                    ASTExpr::Number(3),
                ])),
                ASTPostfixOp::Subscript(vec![ASTExpr::Number(0)]),
            ),
            ASTExpr::PostfixExpr(
                Box::new(ASTExpr::Float(1.1)),
                ASTPostfixOp::Subscript(vec![ASTExpr::Number(0)]),
            ),
            ASTExpr::PostfixExpr(
                Box::new(ASTExpr::Dictionary(vec![(
                    ASTExpr::String("foo"),
                    ASTExpr::String("bar"),
                )])),
                ASTPostfixOp::Subscript(vec![ASTExpr::String("foo")]),
            ),
        ];

        for (input, output) in inputs.into_iter().zip(outputs) {
            let mut parser = create_parser(input);

            assert_eq!(parse_expr(&mut parser), output);
        }
    }

    #[test]
    fn test_parse_binary_exprs() {
        let inputs = [
            "1 + 2",
            "1 - 2",
            "1 * 2",
            "1 / 2",
            "1 % 2",
            "1 << 2",
            "1 >> 2",
            "1 | 2",
            "1 & 2",
            "1 ^ 2",
            "1 == 2",
            "1 != 2",
            "1 > 2",
            "1 >= 2",
            "1 < 2",
            "1 <= 2",
            "1 and 2",
            "1 or 2",
            "1 && 2",
            "1 || 2",
            "1 if 2 else 3",
        ];
        let outputs = [
            ASTExpr::BinaryExpr(
                Box::new(ASTExpr::Number(1)),
                ASTBinaryOp::Add,
                Box::new(ASTExpr::Number(2)),
            ),
            ASTExpr::BinaryExpr(
                Box::new(ASTExpr::Number(1)),
                ASTBinaryOp::Subtract,
                Box::new(ASTExpr::Number(2)),
            ),
            ASTExpr::BinaryExpr(
                Box::new(ASTExpr::Number(1)),
                ASTBinaryOp::Multiply,
                Box::new(ASTExpr::Number(2)),
            ),
            ASTExpr::BinaryExpr(
                Box::new(ASTExpr::Number(1)),
                ASTBinaryOp::Divide,
                Box::new(ASTExpr::Number(2)),
            ),
            ASTExpr::BinaryExpr(
                Box::new(ASTExpr::Number(1)),
                ASTBinaryOp::Remainder,
                Box::new(ASTExpr::Number(2)),
            ),
            ASTExpr::BinaryExpr(
                Box::new(ASTExpr::Number(1)),
                ASTBinaryOp::BitwiseShiftLeft,
                Box::new(ASTExpr::Number(2)),
            ),
            ASTExpr::BinaryExpr(
                Box::new(ASTExpr::Number(1)),
                ASTBinaryOp::BitwiseShiftRight,
                Box::new(ASTExpr::Number(2)),
            ),
            ASTExpr::BinaryExpr(
                Box::new(ASTExpr::Number(1)),
                ASTBinaryOp::BitwiseOr,
                Box::new(ASTExpr::Number(2)),
            ),
            ASTExpr::BinaryExpr(
                Box::new(ASTExpr::Number(1)),
                ASTBinaryOp::BitwiseAnd,
                Box::new(ASTExpr::Number(2)),
            ),
            ASTExpr::BinaryExpr(
                Box::new(ASTExpr::Number(1)),
                ASTBinaryOp::BitwiseXor,
                Box::new(ASTExpr::Number(2)),
            ),
            ASTExpr::BinaryExpr(
                Box::new(ASTExpr::Number(1)),
                ASTBinaryOp::Equals,
                Box::new(ASTExpr::Number(2)),
            ),
            ASTExpr::BinaryExpr(
                Box::new(ASTExpr::Number(1)),
                ASTBinaryOp::NotEqual,
                Box::new(ASTExpr::Number(2)),
            ),
            ASTExpr::BinaryExpr(
                Box::new(ASTExpr::Number(1)),
                ASTBinaryOp::Greater,
                Box::new(ASTExpr::Number(2)),
            ),
            ASTExpr::BinaryExpr(
                Box::new(ASTExpr::Number(1)),
                ASTBinaryOp::GreaterOrEqual,
                Box::new(ASTExpr::Number(2)),
            ),
            ASTExpr::BinaryExpr(
                Box::new(ASTExpr::Number(1)),
                ASTBinaryOp::LessThan,
                Box::new(ASTExpr::Number(2)),
            ),
            ASTExpr::BinaryExpr(
                Box::new(ASTExpr::Number(1)),
                ASTBinaryOp::LessOrEqual,
                Box::new(ASTExpr::Number(2)),
            ),
            ASTExpr::BinaryExpr(
                Box::new(ASTExpr::Number(1)),
                ASTBinaryOp::And,
                Box::new(ASTExpr::Number(2)),
            ),
            ASTExpr::BinaryExpr(
                Box::new(ASTExpr::Number(1)),
                ASTBinaryOp::Or,
                Box::new(ASTExpr::Number(2)),
            ),
            ASTExpr::BinaryExpr(
                Box::new(ASTExpr::Number(1)),
                ASTBinaryOp::And,
                Box::new(ASTExpr::Number(2)),
            ),
            ASTExpr::BinaryExpr(
                Box::new(ASTExpr::Number(1)),
                ASTBinaryOp::Or,
                Box::new(ASTExpr::Number(2)),
            ),
            ASTExpr::BinaryExpr(
                Box::new(ASTExpr::Number(1)),
                ASTBinaryOp::TernaryIfElse(Box::new(ASTExpr::Number(2))),
                Box::new(ASTExpr::Number(3)),
            ),
        ];

        for (input, output) in inputs.into_iter().zip(outputs) {
            let mut parser = create_parser(input);

            assert_eq!(parse_expr(&mut parser), output);
        }
    }

    #[test]
    fn test_expr_associativity() {
        let inputs = [
            "1 + 2 - 3",
            "1 * 2 / 3 % 4",
            "1 << 2 >> 3",
            "1 | 2 | 3",
            "1 & 2 & 3",
            "1 ^ 2 ^ 3",
            "1 == 2 != 3 > 4 >= 5 < 6 <= 7",
            "1 and 2 and 3",
            "1 or 2 or 3",
            "1 && 2 && 3",
            "1 || 2 || 3",
            "1 if 2 else 3 if 4 else 5",
        ];

        let outputs = [
            // 1 + 2 - 3
            ASTExpr::BinaryExpr(
                Box::new(ASTExpr::BinaryExpr(
                    Box::new(ASTExpr::Number(1)),
                    ASTBinaryOp::Add,
                    Box::new(ASTExpr::Number(2)),
                )),
                ASTBinaryOp::Subtract,
                Box::new(ASTExpr::Number(3)),
            ),
            // 1 * 2 / 3 % 4
            ASTExpr::BinaryExpr(
                Box::new(ASTExpr::BinaryExpr(
                    Box::new(ASTExpr::BinaryExpr(
                        Box::new(ASTExpr::Number(1)),
                        ASTBinaryOp::Multiply,
                        Box::new(ASTExpr::Number(2)),
                    )),
                    ASTBinaryOp::Divide,
                    Box::new(ASTExpr::Number(3)),
                )),
                ASTBinaryOp::Remainder,
                Box::new(ASTExpr::Number(4)),
            ),
            // 1 << 2 >> 3
            ASTExpr::BinaryExpr(
                Box::new(ASTExpr::BinaryExpr(
                    Box::new(ASTExpr::Number(1)),
                    ASTBinaryOp::BitwiseShiftLeft,
                    Box::new(ASTExpr::Number(2)),
                )),
                ASTBinaryOp::BitwiseShiftRight,
                Box::new(ASTExpr::Number(3)),
            ),
            // 1 | 2 | 3
            ASTExpr::BinaryExpr(
                Box::new(ASTExpr::BinaryExpr(
                    Box::new(ASTExpr::Number(1)),
                    ASTBinaryOp::BitwiseOr,
                    Box::new(ASTExpr::Number(2)),
                )),
                ASTBinaryOp::BitwiseOr,
                Box::new(ASTExpr::Number(3)),
            ),
            // 1 & 2 & 3
            ASTExpr::BinaryExpr(
                Box::new(ASTExpr::BinaryExpr(
                    Box::new(ASTExpr::Number(1)),
                    ASTBinaryOp::BitwiseAnd,
                    Box::new(ASTExpr::Number(2)),
                )),
                ASTBinaryOp::BitwiseAnd,
                Box::new(ASTExpr::Number(3)),
            ),
            // 1 ^ 2 ^ 3
            ASTExpr::BinaryExpr(
                Box::new(ASTExpr::BinaryExpr(
                    Box::new(ASTExpr::Number(1)),
                    ASTBinaryOp::BitwiseXor,
                    Box::new(ASTExpr::Number(2)),
                )),
                ASTBinaryOp::BitwiseXor,
                Box::new(ASTExpr::Number(3)),
            ),
            // 1 == 2 != 3 > 4 >= 5 < 6 <= 7
            ASTExpr::BinaryExpr(
                Box::new(ASTExpr::BinaryExpr(
                    Box::new(ASTExpr::BinaryExpr(
                        Box::new(ASTExpr::BinaryExpr(
                            Box::new(ASTExpr::BinaryExpr(
                                Box::new(ASTExpr::BinaryExpr(
                                    Box::new(ASTExpr::Number(1)),
                                    ASTBinaryOp::Equals,
                                    Box::new(ASTExpr::Number(2)),
                                )),
                                ASTBinaryOp::NotEqual,
                                Box::new(ASTExpr::Number(3)),
                            )),
                            ASTBinaryOp::Greater,
                            Box::new(ASTExpr::Number(4)),
                        )),
                        ASTBinaryOp::GreaterOrEqual,
                        Box::new(ASTExpr::Number(5)),
                    )),
                    ASTBinaryOp::LessThan,
                    Box::new(ASTExpr::Number(6)),
                )),
                ASTBinaryOp::LessOrEqual,
                Box::new(ASTExpr::Number(7)),
            ),
            // 1 and 2 and 3
            ASTExpr::BinaryExpr(
                Box::new(ASTExpr::BinaryExpr(
                    Box::new(ASTExpr::Number(1)),
                    ASTBinaryOp::And,
                    Box::new(ASTExpr::Number(2)),
                )),
                ASTBinaryOp::And,
                Box::new(ASTExpr::Number(3)),
            ),
            // 1 or 2 or 3
            ASTExpr::BinaryExpr(
                Box::new(ASTExpr::BinaryExpr(
                    Box::new(ASTExpr::Number(1)),
                    ASTBinaryOp::Or,
                    Box::new(ASTExpr::Number(2)),
                )),
                ASTBinaryOp::Or,
                Box::new(ASTExpr::Number(3)),
            ),
            // 1 && 2 && 3
            ASTExpr::BinaryExpr(
                Box::new(ASTExpr::BinaryExpr(
                    Box::new(ASTExpr::Number(1)),
                    ASTBinaryOp::And,
                    Box::new(ASTExpr::Number(2)),
                )),
                ASTBinaryOp::And,
                Box::new(ASTExpr::Number(3)),
            ),
            // 1 || 2 || 3
            ASTExpr::BinaryExpr(
                Box::new(ASTExpr::BinaryExpr(
                    Box::new(ASTExpr::Number(1)),
                    ASTBinaryOp::Or,
                    Box::new(ASTExpr::Number(2)),
                )),
                ASTBinaryOp::Or,
                Box::new(ASTExpr::Number(3)),
            ),
            // 1 if 2 else 3 if 4 else 5
            ASTExpr::BinaryExpr(
                Box::new(ASTExpr::Number(1)),
                ASTBinaryOp::TernaryIfElse(Box::new(ASTExpr::Number(2))),
                Box::new(ASTExpr::BinaryExpr(
                    Box::new(ASTExpr::Number(3)),
                    ASTBinaryOp::TernaryIfElse(Box::new(ASTExpr::Number(4))),
                    Box::new(ASTExpr::Number(5)),
                )),
            ),
        ];

        for (input, output) in inputs.into_iter().zip(outputs) {
            let mut parser = create_parser(input);

            assert_eq!(parse_expr(&mut parser), output);
        }
    }

    #[test]
    fn test_expr_precedence_add_multiply() {
        let mut parser = create_parser("1 + 2 * 3");
        let expected = ASTExpr::BinaryExpr(
            Box::new(ASTExpr::Number(1)),
            ASTBinaryOp::Add,
            Box::new(ASTExpr::BinaryExpr(
                Box::new(ASTExpr::Number(2)),
                ASTBinaryOp::Multiply,
                Box::new(ASTExpr::Number(3)),
            )),
        );
        let result = parse_expr(&mut parser);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_expr_precedence_multiply_add() {
        let mut parser = create_parser("2 * 3 + 1");
        let expected = ASTExpr::BinaryExpr(
            Box::new(ASTExpr::BinaryExpr(
                Box::new(ASTExpr::Number(2)),
                ASTBinaryOp::Multiply,
                Box::new(ASTExpr::Number(3)),
            )),
            ASTBinaryOp::Add,
            Box::new(ASTExpr::Number(1)),
        );
        let result = parse_expr(&mut parser);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_expr_precedence_compare_and() {
        let mut parser = create_parser("1 < 2 && 3 == 4");
        let expected = ASTExpr::BinaryExpr(
            Box::new(ASTExpr::BinaryExpr(
                Box::new(ASTExpr::Number(1)),
                ASTBinaryOp::LessThan,
                Box::new(ASTExpr::Number(2)),
            )),
            ASTBinaryOp::And,
            Box::new(ASTExpr::BinaryExpr(
                Box::new(ASTExpr::Number(3)),
                ASTBinaryOp::Equals,
                Box::new(ASTExpr::Number(4)),
            )),
        );
        let result = parse_expr(&mut parser);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_expr_precedence_compare_or() {
        let mut parser = create_parser("1 < 2 || 3 == 4");
        let expected = ASTExpr::BinaryExpr(
            Box::new(ASTExpr::BinaryExpr(
                Box::new(ASTExpr::Number(1)),
                ASTBinaryOp::LessThan,
                Box::new(ASTExpr::Number(2)),
            )),
            ASTBinaryOp::Or,
            Box::new(ASTExpr::BinaryExpr(
                Box::new(ASTExpr::Number(3)),
                ASTBinaryOp::Equals,
                Box::new(ASTExpr::Number(4)),
            )),
        );
        let result = parse_expr(&mut parser);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_expr_precedence_assignment() {
        let mut parser = create_parser("a = b = c = 42");
        let expected = ASTExpr::BinaryExpr(
            Box::new(ASTExpr::BinaryExpr(
                Box::new(ASTExpr::BinaryExpr(
                    Box::new(ASTExpr::Identifier("a")),
                    ASTBinaryOp::Assignment,
                    Box::new(ASTExpr::Identifier("b")),
                )),
                ASTBinaryOp::Assignment,
                Box::new(ASTExpr::Identifier("c")),
            )),
            ASTBinaryOp::Assignment,
            Box::new(ASTExpr::Number(42)),
        );
        let result = parse_expr(&mut parser);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_expr_group_newlines() {
        let mut parser = create_parser("(1\n+ fn\n())");
        let expected = ASTExpr::Group(vec![ASTExpr::BinaryExpr(
            Box::new(ASTExpr::Number(1)),
            ASTBinaryOp::Add,
            Box::new(ASTExpr::PostfixExpr(
                Box::new(ASTExpr::Identifier("fn")),
                ASTPostfixOp::Call(vec![]),
            )),
        )]);
        let result = parse_expr(&mut parser);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_expr_group_multiple() {
        let mut parser = create_parser("(1, 2)");
        let expected = ASTExpr::Group(vec![ASTExpr::Number(1), ASTExpr::Number(2)]);
        let result = parse_expr(&mut parser);

        assert_eq!(result, expected);
    }
}
