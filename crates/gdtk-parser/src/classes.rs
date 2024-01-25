use gdtk_lexer::Token;
use gdtk_ast::poor::{ASTEnum, ASTEnumVariant};

use crate::values::parse_value;
use crate::utils::{expect_blank_prefixed, next_non_blank};

pub fn parse_classname<'a, T>(iter: &mut T) -> &'a str
where
    T: Iterator<Item = Token<'a>>,
{
    expect_blank_prefixed!(iter, Token::Identifier(i), i)
}

pub fn parse_extends<'a, T>(iter: &mut T) -> &'a str
where
    T: Iterator<Item = Token<'a>>,
{
    expect_blank_prefixed!(iter, Token::Identifier(s), s)
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
