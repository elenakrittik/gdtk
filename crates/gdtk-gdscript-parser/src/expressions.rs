use gdtk_ast::{
    ASTBinaryOp, ASTExpr, ASTExprKind, ASTPostfixOp, ASTPostfixOpKind, ASTPrefixOp, ASTPrefixOpKind,
};
use crate::lexer::{Token, TokenKind};
use gdtk_span::Span;
use pratt::{Affix, Associativity, PrattParser, Precedence};

use crate::{
    utils::{advance_and_parse, delemited_by, expect},
    values::{parse_array, parse_dictionary, parse_lambda},
    Parser,
};

/// Parse an expression.
pub fn parse_expr<'a>(parser: &mut Parser<'a, impl Iterator<Item = Token<'a>>>) -> ASTExpr<'a> {
    ExprParser
        .parse(parse_expr_impl(parser).into_iter())
        .unwrap()
}

fn parse_expr_impl<'a>(
    parser: &mut Parser<'a, impl Iterator<Item = Token<'a>>>,
) -> Vec<ExprIR<'a>> {
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
    parser: &mut Parser<'a, impl Iterator<Item = Token<'a>>>,
) -> Vec<ExprIR<'a>> {
    let mut result = vec![];

    while let Some(op) = match parser.peek().map(|t| &t.kind) {
        Some(TokenKind::Plus) => Some(ASTPrefixOpKind::Identity),
        Some(TokenKind::Minus) => Some(ASTPrefixOpKind::Negation),
        Some(TokenKind::Await) => Some(ASTPrefixOpKind::Await),
        Some(TokenKind::BitwiseNot) => Some(ASTPrefixOpKind::BitwiseNot),
        Some(TokenKind::Not | TokenKind::SymbolizedNot) => Some(ASTPrefixOpKind::Not),
        None => panic!("expected expression"),
        _ => None,
    } {
        let span = parser.next().unwrap().span;
        result.push(ExprIR::Prefix(ASTPrefixOp { kind: op, span }));
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

        let start = parser.span_start();

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

                let span = parser.finish_span(start);

                result.push(ExprIR::Postfix(ASTPostfixOp {
                    kind: ASTPostfixOpKind::Call(values),
                    span,
                }));
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

                let span = parser.finish_span(start);

                result.push(ExprIR::Postfix(ASTPostfixOp {
                    kind: ASTPostfixOpKind::Subscript(values),
                    span,
                }));
            }
            _ => unreachable!(),
        }
    }

    result
}

/// Parse a "clean" value, without checking for possible prefix or postfix OPs
fn parse_expr_without_ops<'a>(
    parser: &mut Parser<'a, impl Iterator<Item = Token<'a>>>,
) -> ExprIR<'a> {
    let start = parser.span_start();

    #[rustfmt::skip]
    let kind = match &parser.peek().expect("unexpected EOF").kind {
        TokenKind::Identifier(_) => ASTExprKind::Identifier(expect!(parser, TokenKind::Identifier(s), s))
        ,TokenKind::Integer(_) => ASTExprKind::Number(expect!(parser, TokenKind::Integer(n), n)),
        TokenKind::BinaryInteger(_) => ASTExprKind::Number(expect!(parser, TokenKind::BinaryInteger(n), n)),
        TokenKind::HexInteger(_) => ASTExprKind::Number(expect!(parser, TokenKind::HexInteger(n), n)),
        TokenKind::Float(_) => ASTExprKind::Float(expect!(parser, TokenKind::Float(f), f)),
        TokenKind::ScientificFloat(_) => ASTExprKind::Float(expect!(parser, TokenKind::ScientificFloat(f), f)),
        TokenKind::String(_) => ASTExprKind::String(expect!(parser, TokenKind::String(s), s)),
        TokenKind::StringName(_) => ASTExprKind::StringName(expect!(parser, TokenKind::StringName(s), s)),
        TokenKind::Node(_) => ASTExprKind::Node(expect!(parser, TokenKind::Node(s), s)),
        TokenKind::UniqueNode(_) => ASTExprKind::UniqueNode(expect!(parser, TokenKind::UniqueNode(s), s)),
        TokenKind::NodePath(_) => ASTExprKind::NodePath(expect!(parser, TokenKind::NodePath(s), s)),
        TokenKind::Boolean(_) => ASTExprKind::Boolean(expect!(parser, TokenKind::Boolean(b), b)),
        TokenKind::Func => ASTExprKind::Lambda(parse_lambda(parser)),
        TokenKind::OpeningBracket => ASTExprKind::Array(parse_array(parser)),
        TokenKind::OpeningBrace => ASTExprKind::Dictionary(parse_dictionary(parser)),
        TokenKind::OpeningParenthesis => {
            let start = parser.span_start();

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

            return ExprIR::Group(values, parser.finish_span(start));
        }
        TokenKind::Null => advance_and_parse(parser, |_| ASTExprKind::Null),
        _ => panic!("unknown or unsupported expression: {:#?}", parser.peek()),
    };

    ExprIR::Primary(ASTExpr {
        kind,
        span: parser.finish_span(start),
    })
}

#[derive(Debug, enum_as_inner::EnumAsInner)]
enum ExprIR<'a> {
    Prefix(ASTPrefixOp),
    Postfix(ASTPostfixOp<'a>),
    Binary(ASTBinaryOp<'a>),
    Group(Vec<ASTExpr<'a>>, Span),
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
            ExprIR::Group(_, _) => Affix::Nilfix,

            ExprIR::Postfix(ASTPostfixOp { kind: ASTPostfixOpKind::Subscript(_), .. }) => Affix::Postfix(Precedence(23)),

            ExprIR::Binary(ASTBinaryOp::PropertyAccess) => Affix::Infix(Precedence(22), Associativity::Left),

            ExprIR::Postfix(ASTPostfixOp { kind: ASTPostfixOpKind::Call(_), .. }) => Affix::Postfix(Precedence(21)),

            ExprIR::Prefix(ASTPrefixOp { kind: ASTPrefixOpKind::Await, .. }) => Affix::Prefix(Precedence(20)),

            ExprIR::Binary(ASTBinaryOp::TypeCheck) => Affix::Infix(Precedence(19), Associativity::Left),
            ExprIR::Binary(ASTBinaryOp::Power) => Affix::Infix(Precedence(18), Associativity::Left),

            ExprIR::Prefix(ASTPrefixOp { kind: ASTPrefixOpKind::BitwiseNot, .. }) => Affix::Prefix(Precedence(17)),

            ExprIR::Prefix(ASTPrefixOp { kind: ASTPrefixOpKind::Identity, .. }) => Affix::Prefix(Precedence(16)),
            ExprIR::Prefix(ASTPrefixOp { kind: ASTPrefixOpKind::Negation, .. }) => Affix::Prefix(Precedence(16)),

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

            ExprIR::Prefix(ASTPrefixOp { kind: ASTPrefixOpKind::Not, .. }) => Affix::Prefix(Precedence(7)),

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
            ExprIR::Group(vals, span) => ASTExpr {
                kind: ASTExprKind::Group(vals),
                span,
            },
            _ => unreachable!(),
        })
    }

    fn infix(
        &mut self,
        lhs: Self::Output,
        op: Self::Input,
        rhs: Self::Output,
    ) -> Result<Self::Output, Self::Error> {
        let span = lhs.span.start..rhs.span.end;

        Ok(ASTExpr {
            kind: ASTExprKind::BinaryExpr(Box::new(lhs), op.into_binary().unwrap(), Box::new(rhs)),
            span,
        })
    }

    fn prefix(&mut self, op: Self::Input, rhs: Self::Output) -> Result<Self::Output, Self::Error> {
        let op = op.into_prefix().unwrap();
        let span = op.span.start..rhs.span.end;

        Ok(ASTExpr {
            kind: ASTExprKind::PrefixExpr(op, Box::new(rhs)),
            span,
        })
    }

    fn postfix(&mut self, lhs: Self::Output, op: Self::Input) -> Result<Self::Output, Self::Error> {
        let op = op.into_postfix().unwrap();

        let span = lhs.span.start..op.span.end;

        Ok(ASTExpr {
            kind: ASTExprKind::PostfixExpr(Box::new(lhs), op),
            span,
        })
    }
}

#[cfg(test)]
mod tests {
    use gdtk_ast::*;

    use crate::expressions::parse_expr;
    use crate::test_utils::{create_parser, make_ident, make_number, make_string};

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
            ASTExprKind::Identifier("ident"),
            ASTExprKind::Number(123556789),
            ASTExprKind::Number(0x123abc),
            ASTExprKind::Float(0.123456789),
            ASTExprKind::Float(1.0e2),
            ASTExprKind::String("string"),
            ASTExprKind::StringName("string"),
            ASTExprKind::Node("string"),
            ASTExprKind::UniqueNode("string"),
            ASTExprKind::NodePath("string"),
            ASTExprKind::Boolean(true),
            ASTExprKind::Boolean(false),
            ASTExprKind::Null,
        ];

        for (input, output) in inputs.into_iter().zip(outputs) {
            let mut parser = create_parser(input);

            assert_eq!(parse_expr(&mut parser).kind, output);
        }
    }

    #[test]
    fn test_parse_prefix_exprs() {
        let inputs = ["await 1", "+1", "-1", "not 1", "!1", "~1"];
        let outputs = [
            ASTExprKind::PrefixExpr(
                ASTPrefixOp {
                    kind: ASTPrefixOpKind::Await,
                    span: 0..0,
                },
                Box::new(make_number(1)),
            ),
            ASTExprKind::PrefixExpr(
                ASTPrefixOp {
                    kind: ASTPrefixOpKind::Identity,
                    span: 0..0,
                },
                Box::new(make_number(1)),
            ),
            ASTExprKind::PrefixExpr(
                ASTPrefixOp {
                    kind: ASTPrefixOpKind::Negation,
                    span: 0..0,
                },
                Box::new(make_number(1)),
            ),
            ASTExprKind::PrefixExpr(
                ASTPrefixOp {
                    kind: ASTPrefixOpKind::Not,
                    span: 0..0,
                },
                Box::new(make_number(1)),
            ),
            ASTExprKind::PrefixExpr(
                ASTPrefixOp {
                    kind: ASTPrefixOpKind::Not,
                    span: 0..0,
                },
                Box::new(make_number(1)),
            ),
            ASTExprKind::PrefixExpr(
                ASTPrefixOp {
                    kind: ASTPrefixOpKind::BitwiseNot,
                    span: 0..0,
                },
                Box::new(make_number(1)),
            ),
        ];

        for (input, output) in inputs.into_iter().zip(outputs) {
            let mut parser = create_parser(input);

            assert_eq!(parse_expr(&mut parser).kind, output);
        }
    }

    #[test]
    fn test_parse_postfix_exprs() {
        let inputs = ["1(2, 3)", "[1, 2, 3][0]", "1.1[0]", "{'foo': 'bar'}['foo']"];
        let outputs = [
            ASTExprKind::PostfixExpr(
                Box::new(make_number(1)),
                ASTPostfixOp {
                    kind: ASTPostfixOpKind::Call(vec![make_number(2), make_number(3)]),
                    span: 0..0,
                },
            ),
            ASTExprKind::PostfixExpr(
                Box::new(ASTExpr {
                    kind: ASTExprKind::Array(vec![make_number(1), make_number(2), make_number(3)]),
                    span: 0..0,
                }),
                ASTPostfixOp {
                    kind: ASTPostfixOpKind::Subscript(vec![make_number(0)]),
                    span: 0..0,
                },
            ),
            ASTExprKind::PostfixExpr(
                Box::new(ASTExpr {
                    kind: ASTExprKind::Float(1.1),
                    span: 0..0,
                }),
                ASTPostfixOp {
                    kind: ASTPostfixOpKind::Subscript(vec![make_number(0)]),
                    span: 0..0,
                },
            ),
            ASTExprKind::PostfixExpr(
                Box::new(ASTExpr {
                    kind: ASTExprKind::Dictionary(vec![(make_string("foo"), make_string("bar"))]),
                    span: 0..0,
                }),
                ASTPostfixOp {
                    kind: ASTPostfixOpKind::Subscript(vec![make_string("foo")]),
                    span: 0..0,
                },
            ),
        ];

        for (input, output) in inputs.into_iter().zip(outputs) {
            let mut parser = create_parser(input);

            assert_eq!(parse_expr(&mut parser).kind, output);
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
            ASTExprKind::BinaryExpr(
                Box::new(make_number(1)),
                ASTBinaryOp::Add,
                Box::new(make_number(2)),
            ),
            ASTExprKind::BinaryExpr(
                Box::new(make_number(1)),
                ASTBinaryOp::Subtract,
                Box::new(make_number(2)),
            ),
            ASTExprKind::BinaryExpr(
                Box::new(make_number(1)),
                ASTBinaryOp::Multiply,
                Box::new(make_number(2)),
            ),
            ASTExprKind::BinaryExpr(
                Box::new(make_number(1)),
                ASTBinaryOp::Divide,
                Box::new(make_number(2)),
            ),
            ASTExprKind::BinaryExpr(
                Box::new(make_number(1)),
                ASTBinaryOp::Remainder,
                Box::new(make_number(2)),
            ),
            ASTExprKind::BinaryExpr(
                Box::new(make_number(1)),
                ASTBinaryOp::BitwiseShiftLeft,
                Box::new(make_number(2)),
            ),
            ASTExprKind::BinaryExpr(
                Box::new(make_number(1)),
                ASTBinaryOp::BitwiseShiftRight,
                Box::new(make_number(2)),
            ),
            ASTExprKind::BinaryExpr(
                Box::new(make_number(1)),
                ASTBinaryOp::BitwiseOr,
                Box::new(make_number(2)),
            ),
            ASTExprKind::BinaryExpr(
                Box::new(make_number(1)),
                ASTBinaryOp::BitwiseAnd,
                Box::new(make_number(2)),
            ),
            ASTExprKind::BinaryExpr(
                Box::new(make_number(1)),
                ASTBinaryOp::BitwiseXor,
                Box::new(make_number(2)),
            ),
            ASTExprKind::BinaryExpr(
                Box::new(make_number(1)),
                ASTBinaryOp::Equals,
                Box::new(make_number(2)),
            ),
            ASTExprKind::BinaryExpr(
                Box::new(make_number(1)),
                ASTBinaryOp::NotEqual,
                Box::new(make_number(2)),
            ),
            ASTExprKind::BinaryExpr(
                Box::new(make_number(1)),
                ASTBinaryOp::Greater,
                Box::new(make_number(2)),
            ),
            ASTExprKind::BinaryExpr(
                Box::new(make_number(1)),
                ASTBinaryOp::GreaterOrEqual,
                Box::new(make_number(2)),
            ),
            ASTExprKind::BinaryExpr(
                Box::new(make_number(1)),
                ASTBinaryOp::LessThan,
                Box::new(make_number(2)),
            ),
            ASTExprKind::BinaryExpr(
                Box::new(make_number(1)),
                ASTBinaryOp::LessOrEqual,
                Box::new(make_number(2)),
            ),
            ASTExprKind::BinaryExpr(
                Box::new(make_number(1)),
                ASTBinaryOp::And,
                Box::new(make_number(2)),
            ),
            ASTExprKind::BinaryExpr(
                Box::new(make_number(1)),
                ASTBinaryOp::Or,
                Box::new(make_number(2)),
            ),
            ASTExprKind::BinaryExpr(
                Box::new(make_number(1)),
                ASTBinaryOp::And,
                Box::new(make_number(2)),
            ),
            ASTExprKind::BinaryExpr(
                Box::new(make_number(1)),
                ASTBinaryOp::Or,
                Box::new(make_number(2)),
            ),
            ASTExprKind::BinaryExpr(
                Box::new(make_number(1)),
                ASTBinaryOp::TernaryIfElse(Box::new(make_number(2))),
                Box::new(make_number(3)),
            ),
        ];

        for (input, output) in inputs.into_iter().zip(outputs) {
            let mut parser = create_parser(input);

            assert_eq!(parse_expr(&mut parser).kind, output);
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
            ASTExprKind::BinaryExpr(
                Box::new(ASTExpr {
                    kind: ASTExprKind::BinaryExpr(
                        Box::new(make_number(1)),
                        ASTBinaryOp::Add,
                        Box::new(make_number(2)),
                    ),
                    span: 0..0,
                }),
                ASTBinaryOp::Subtract,
                Box::new(make_number(3)),
            ),
            // 1 * 2 / 3 % 4
            ASTExprKind::BinaryExpr(
                Box::new(ASTExpr {
                    kind: ASTExprKind::BinaryExpr(
                        Box::new(ASTExpr {
                            kind: ASTExprKind::BinaryExpr(
                                Box::new(make_number(1)),
                                ASTBinaryOp::Multiply,
                                Box::new(make_number(2)),
                            ),
                            span: 0..0,
                        }),
                        ASTBinaryOp::Divide,
                        Box::new(make_number(3)),
                    ),
                    span: 0..0,
                }),
                ASTBinaryOp::Remainder,
                Box::new(make_number(4)),
            ),
            // 1 << 2 >> 3
            ASTExprKind::BinaryExpr(
                Box::new(ASTExpr {
                    kind: ASTExprKind::BinaryExpr(
                        Box::new(make_number(1)),
                        ASTBinaryOp::BitwiseShiftLeft,
                        Box::new(make_number(2)),
                    ),
                    span: 0..0,
                }),
                ASTBinaryOp::BitwiseShiftRight,
                Box::new(make_number(3)),
            ),
            // 1 | 2 | 3
            ASTExprKind::BinaryExpr(
                Box::new(ASTExpr {
                    kind: ASTExprKind::BinaryExpr(
                        Box::new(make_number(1)),
                        ASTBinaryOp::BitwiseOr,
                        Box::new(make_number(2)),
                    ),
                    span: 0..0,
                }),
                ASTBinaryOp::BitwiseOr,
                Box::new(make_number(3)),
            ),
            // 1 & 2 & 3
            ASTExprKind::BinaryExpr(
                Box::new(ASTExpr {
                    kind: ASTExprKind::BinaryExpr(
                        Box::new(make_number(1)),
                        ASTBinaryOp::BitwiseAnd,
                        Box::new(make_number(2)),
                    ),
                    span: 0..0,
                }),
                ASTBinaryOp::BitwiseAnd,
                Box::new(make_number(3)),
            ),
            // 1 ^ 2 ^ 3
            ASTExprKind::BinaryExpr(
                Box::new(ASTExpr {
                    kind: ASTExprKind::BinaryExpr(
                        Box::new(make_number(1)),
                        ASTBinaryOp::BitwiseXor,
                        Box::new(make_number(2)),
                    ),
                    span: 0..0,
                }),
                ASTBinaryOp::BitwiseXor,
                Box::new(make_number(3)),
            ),
            // 1 == 2 != 3 > 4 >= 5 < 6 <= 7
            ASTExprKind::BinaryExpr(
                Box::new(ASTExpr {
                    kind: ASTExprKind::BinaryExpr(
                        Box::new(ASTExpr {
                            kind: ASTExprKind::BinaryExpr(
                                Box::new(ASTExpr {
                                    kind: ASTExprKind::BinaryExpr(
                                        Box::new(ASTExpr {
                                            kind: ASTExprKind::BinaryExpr(
                                                Box::new(ASTExpr {
                                                    kind: ASTExprKind::BinaryExpr(
                                                        Box::new(make_number(1)),
                                                        ASTBinaryOp::Equals,
                                                        Box::new(make_number(2)),
                                                    ),
                                                    span: 0..0,
                                                }),
                                                ASTBinaryOp::NotEqual,
                                                Box::new(make_number(3)),
                                            ),
                                            span: 0..0,
                                        }),
                                        ASTBinaryOp::Greater,
                                        Box::new(make_number(4)),
                                    ),
                                    span: 0..0,
                                }),
                                ASTBinaryOp::GreaterOrEqual,
                                Box::new(make_number(5)),
                            ),
                            span: 0..0,
                        }),
                        ASTBinaryOp::LessThan,
                        Box::new(make_number(6)),
                    ),
                    span: 0..0,
                }),
                ASTBinaryOp::LessOrEqual,
                Box::new(make_number(7)),
            ),
            // 1 and 2 and 3
            ASTExprKind::BinaryExpr(
                Box::new(ASTExpr {
                    kind: ASTExprKind::BinaryExpr(
                        Box::new(make_number(1)),
                        ASTBinaryOp::And,
                        Box::new(make_number(2)),
                    ),
                    span: 0..0,
                }),
                ASTBinaryOp::And,
                Box::new(make_number(3)),
            ),
            // 1 or 2 or 3
            ASTExprKind::BinaryExpr(
                Box::new(ASTExpr {
                    kind: ASTExprKind::BinaryExpr(
                        Box::new(make_number(1)),
                        ASTBinaryOp::Or,
                        Box::new(make_number(2)),
                    ),
                    span: 0..0,
                }),
                ASTBinaryOp::Or,
                Box::new(make_number(3)),
            ),
            // 1 && 2 && 3
            ASTExprKind::BinaryExpr(
                Box::new(ASTExpr {
                    kind: ASTExprKind::BinaryExpr(
                        Box::new(make_number(1)),
                        ASTBinaryOp::And,
                        Box::new(make_number(2)),
                    ),
                    span: 0..0,
                }),
                ASTBinaryOp::And,
                Box::new(make_number(3)),
            ),
            // 1 || 2 || 3
            ASTExprKind::BinaryExpr(
                Box::new(ASTExpr {
                    kind: ASTExprKind::BinaryExpr(
                        Box::new(make_number(1)),
                        ASTBinaryOp::Or,
                        Box::new(make_number(2)),
                    ),
                    span: 0..0,
                }),
                ASTBinaryOp::Or,
                Box::new(make_number(3)),
            ),
            // 1 if 2 else 3 if 4 else 5
            ASTExprKind::BinaryExpr(
                Box::new(make_number(1)),
                ASTBinaryOp::TernaryIfElse(Box::new(make_number(2))),
                Box::new(ASTExpr {
                    kind: ASTExprKind::BinaryExpr(
                        Box::new(make_number(3)),
                        ASTBinaryOp::TernaryIfElse(Box::new(make_number(4))),
                        Box::new(make_number(5)),
                    ),
                    span: 0..0,
                }),
            ),
        ];

        for (input, output) in inputs.into_iter().zip(outputs) {
            let mut parser = create_parser(input);

            assert_eq!(parse_expr(&mut parser).kind, output);
        }
    }

    #[test]
    fn test_expr_precedence_add_multiply() {
        let mut parser = create_parser("1 + 2 * 3");
        let expected = ASTExprKind::BinaryExpr(
            Box::new(make_number(1)),
            ASTBinaryOp::Add,
            Box::new(ASTExpr {
                kind: ASTExprKind::BinaryExpr(
                    Box::new(make_number(2)),
                    ASTBinaryOp::Multiply,
                    Box::new(make_number(3)),
                ),
                span: 0..0,
            }),
        );
        let result = parse_expr(&mut parser);

        assert_eq!(result.kind, expected);
    }

    #[test]
    fn test_expr_precedence_multiply_add() {
        let mut parser = create_parser("2 * 3 + 1");
        let expected = ASTExprKind::BinaryExpr(
            Box::new(ASTExpr {
                kind: ASTExprKind::BinaryExpr(
                    Box::new(make_number(2)),
                    ASTBinaryOp::Multiply,
                    Box::new(make_number(3)),
                ),
                span: 0..0,
            }),
            ASTBinaryOp::Add,
            Box::new(make_number(1)),
        );
        let result = parse_expr(&mut parser);

        assert_eq!(result.kind, expected);
    }

    #[test]
    fn test_expr_precedence_compare_and() {
        let mut parser = create_parser("1 < 2 && 3 == 4");
        let expected = ASTExprKind::BinaryExpr(
            Box::new(ASTExpr {
                kind: ASTExprKind::BinaryExpr(
                    Box::new(make_number(1)),
                    ASTBinaryOp::LessThan,
                    Box::new(make_number(2)),
                ),
                span: 0..0,
            }),
            ASTBinaryOp::And,
            Box::new(ASTExpr {
                kind: ASTExprKind::BinaryExpr(
                    Box::new(make_number(3)),
                    ASTBinaryOp::Equals,
                    Box::new(make_number(4)),
                ),
                span: 0..0,
            }),
        );
        let result = parse_expr(&mut parser);

        assert_eq!(result.kind, expected);
    }

    #[test]
    fn test_expr_precedence_compare_or() {
        let mut parser = create_parser("1 < 2 || 3 == 4");
        let expected = ASTExprKind::BinaryExpr(
            Box::new(ASTExpr {
                kind: ASTExprKind::BinaryExpr(
                    Box::new(make_number(1)),
                    ASTBinaryOp::LessThan,
                    Box::new(make_number(2)),
                ),
                span: 0..0,
            }),
            ASTBinaryOp::Or,
            Box::new(ASTExpr {
                kind: ASTExprKind::BinaryExpr(
                    Box::new(make_number(3)),
                    ASTBinaryOp::Equals,
                    Box::new(make_number(4)),
                ),
                span: 0..0,
            }),
        );
        let result = parse_expr(&mut parser);

        assert_eq!(result.kind, expected);
    }

    #[test]
    fn test_expr_precedence_assignment() {
        let mut parser = create_parser("a = b = c = 42");
        let expected = ASTExprKind::BinaryExpr(
            Box::new(ASTExpr {
                kind: ASTExprKind::BinaryExpr(
                    Box::new(ASTExpr {
                        kind: ASTExprKind::BinaryExpr(
                            Box::new(make_ident("a")),
                            ASTBinaryOp::Assignment,
                            Box::new(make_ident("b")),
                        ),
                        span: 0..0,
                    }),
                    ASTBinaryOp::Assignment,
                    Box::new(make_ident("c")),
                ),
                span: 0..0,
            }),
            ASTBinaryOp::Assignment,
            Box::new(make_number(42)),
        );
        let result = parse_expr(&mut parser);

        assert_eq!(result.kind, expected);
    }

    #[test]
    fn test_expr_group_newlines() {
        let mut parser = create_parser("(1\n+ fn\n())");
        let expected = ASTExprKind::Group(vec![ASTExpr {
            kind: ASTExprKind::BinaryExpr(
                Box::new(make_number(1)),
                ASTBinaryOp::Add,
                Box::new(ASTExpr {
                    kind: ASTExprKind::PostfixExpr(
                        Box::new(make_ident("fn")),
                        ASTPostfixOp {
                            kind: ASTPostfixOpKind::Call(vec![]),
                            span: 0..0,
                        },
                    ),
                    span: 0..0,
                }),
            ),
            span: 0..0,
        }]);
        let result = parse_expr(&mut parser);

        assert_eq!(result.kind, expected);
    }

    #[test]
    fn test_expr_group_multiple() {
        let mut parser = create_parser("(1, 2)");
        let expected = ASTExprKind::Group(vec![make_number(1), make_number(2)]);
        let result = parse_expr(&mut parser);

        assert_eq!(result.kind, expected);
    }
}
