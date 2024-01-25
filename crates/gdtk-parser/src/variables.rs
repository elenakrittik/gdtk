use gdtk_lexer::Token;
use gdtk_ast::poor::{ASTVariable, ASTVariableKind};
use crate::utils::{expect_blank_prefixed, parse_idtydef, next_non_blank};
use crate::values::parse_value;

pub fn parse_const<'a, T>(iter: &mut T) -> ASTVariable<'a>
where
    T: Iterator<Item = Token<'a>>,
{
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
    let (identifier, infer_type, typehint, value) = parse_idtydef!(iter, Token::Newline => (),);

    ASTVariable {
        identifier,
        infer_type,
        typehint,
        value,
        kind: ASTVariableKind::Regular,
    }
}
