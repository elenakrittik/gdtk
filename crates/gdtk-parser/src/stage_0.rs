use gdtk_diag::Diagnostic;
use gdtk_lexer::{error::WithSpan, token::Token, Lexeme};

use crate::error::{Error, IntoDiag};

pub fn run(tokens: Vec<Lexeme>, diags: &mut Vec<Diagnostic>) {
    let mut style = IndentStyle::default();
    let mut out = vec![];
    let mut block_length_stack = vec![];
    let mut just_passed_colon = false;

    for (token, span) in tokens.into_iter() {
        if matches!(token, Token::Colon) {
            just_passed_colon = true;
            out.push((token, span));
            continue;
        }

        if just_passed_colon {
            just_passed_colon = false;

            if let Token::Blank(blank) = token {
                let current_style = match get_style(blank) {
                    Ok(stl) => stl,
                    Err(err) => {
                        diags.push(err.with_span(span).into_diag());
                        continue;
                    }
                };

                if matches!(style, IndentStyle::Unknown) {
                    style = current_style;
                }

                block_length_stack.push(blank.len());
                out.push((Token::Indent, span));
            } else {
                diags.push(Error::Expected("indented block after semicolon".to_owned()).with_span(span).into_diag());
            }

            continue;
        }

        out.push((token, span));
    }

    dbg!(out);
    dbg!(diags);
}

fn get_style(blank: &str) -> Result<IndentStyle, Error> {
    if blank.chars().all(|c| c == ' ') {
        Ok(IndentStyle::Spaces)
    } else if blank.chars().all(|c| c == '\t') {
        Ok(IndentStyle::Tabs)
    } else {
        Err(Error::MixedIndent)
    }
}

#[derive(Default)]
enum IndentStyle {
    #[default]
    Unknown,
    Spaces,
    Tabs,
}
