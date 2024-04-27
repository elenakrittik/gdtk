use gdtk_ast::CodeBlock;

use crate::lexer::{Token, TokenKind};
use crate::statement::parse_statement;
use crate::utils::expect;
use crate::Parser;

pub fn parse_block<'a>(
    parser: &mut Parser<'a, impl Iterator<Item = Token<'a>>>,
    value: bool,
) -> CodeBlock<'a> {
    let mut stmts = vec![];

    // Check if the block is multiline.
    if parser
        .peek()
        .is_some_and(|t| matches!(t.kind, TokenKind::Newline))
    {
        expect!(parser, TokenKind::Newline);
        expect!(parser, TokenKind::Indent);

        while let Some(Token { kind, .. }) = parser.peek() {
            match kind {
                TokenKind::Dedent => {
                    parser.next();
                    break;
                }
                TokenKind::Newline | TokenKind::Semicolon => {
                    parser.next();
                }
                TokenKind::ClosingParenthesis
                | TokenKind::ClosingBracket
                | TokenKind::ClosingBrace => {
                    if value {
                        break;
                    } else {
                        stmts.push(parse_statement(parser));
                    }
                }
                _ => stmts.push(parse_statement(parser)),
            }
        }
    } else {
        stmts.push(parse_statement(parser));
    }

    stmts
}

#[cfg(test)]
mod tests {
    use gdtk_ast::*;

    use crate::block::parse_block;
    use crate::test_utils::create_parser;

    #[test]
    fn test_parse_block_indents() {
        let mut parser = create_parser("\n    pass\n");
        let expected = vec![ASTStatement::Pass(ASTPassStmt { span: 0..0 })];
        let result = parse_block(&mut parser, false);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_block_parens() {
        let mut parser = create_parser("\n    pass)");
        let expected = vec![ASTStatement::Pass(ASTPassStmt { span: 0..0 })];
        let result = parse_block(&mut parser, true);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_block_inline() {
        let mut parser = create_parser("pass");
        let expected = vec![ASTStatement::Pass(ASTPassStmt { span: 0..0 })];
        let result = parse_block(&mut parser, false);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_block_semicolons() {
        let mut parser = create_parser("\n    pass;pass");
        let expected = vec![
            ASTStatement::Pass(ASTPassStmt { span: 0..0 }),
            ASTStatement::Pass(ASTPassStmt { span: 0..0 }),
        ];
        let result = parse_block(&mut parser, false);

        assert_eq!(result, expected);
    }
}
