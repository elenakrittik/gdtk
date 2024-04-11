use std::iter::Peekable;

use gdtk_ast::poor::CodeBlock;
use gdtk_lexer::{Token, TokenKind};

use crate::statement::parse_statement;
use crate::utils::expect;

pub fn parse_block<'a>(
    iter: &mut Peekable<impl Iterator<Item = Token<'a>>>,
    value: bool,
) -> CodeBlock<'a> {
    let mut stmts = vec![];

    // Check if the block is multiline.
    if iter
        .peek()
        .is_some_and(|t| matches!(t.kind, TokenKind::Newline))
    {
        expect!(iter, TokenKind::Newline);
        expect!(iter, TokenKind::Indent);

        while let Some(Token { kind, .. }) = iter.peek() {
            match kind {
                TokenKind::Dedent => {
                    iter.next();
                    break;
                }
                TokenKind::Newline | TokenKind::Semicolon => {
                    iter.next();
                }
                TokenKind::ClosingParenthesis
                | TokenKind::ClosingBracket
                | TokenKind::ClosingBrace => {
                    if value {
                        break;
                    } else {
                        stmts.push(parse_statement(iter));
                    }
                }
                _ => stmts.push(parse_statement(iter)),
            }
        }
    } else {
        stmts.push(parse_statement(iter));
    }

    stmts
}

#[cfg(test)]
mod tests {
    use gdtk_ast::poor::*;

    use crate::block::parse_block;
    use crate::test_utils::create_parser;

    #[test]
    fn test_parse_block_indents() {
        let mut parser = create_parser("\n    pass\n");
        let expected = vec![ASTStatement::Pass];
        let result = parse_block(&mut parser, false);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_block_parens() {
        let mut parser = create_parser("\n    pass)");
        let expected = vec![ASTStatement::Pass];
        let result = parse_block(&mut parser, true);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_block_inline() {
        let mut parser = create_parser("pass");
        let expected = vec![ASTStatement::Pass];
        let result = parse_block(&mut parser, false);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_block_semicolons() {
        let mut parser = create_parser("\n    pass;pass");
        let expected = vec![ASTStatement::Pass, ASTStatement::Pass];
        let result = parse_block(&mut parser, false);

        assert_eq!(result, expected);
    }
}
