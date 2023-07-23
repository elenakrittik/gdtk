use crate::{
    error::{Error, SpannedError},
    spanned,
};
use crate::{
    state::{IndentStyle, State},
    token::Token,
};

// pub fn indent_frontend<'a>(lex: &mut logos::Lexer<'a, Token<'a>>) {}

pub fn check_indent<'a>(lex: &mut logos::Lexer<'a, Token<'a>>) -> Result<&'a str, SpannedError> {
    let indent_style = match check_indent_impl(&lex.extras, lex.slice()) {
        Ok(v) => v,
        Err(e) => return Err(spanned!(lex, e)),
    };
    lex.extras.indent_style = Some(indent_style);

    Ok(lex.slice())
}

fn check_indent_impl(extras: &State, slice: &str) -> Result<IndentStyle, Error> {
    let all_tabs = slice.chars().all(|c| c == '\t');
    let all_spaces = slice.chars().all(|c| c == ' ');

    if !all_spaces && !all_tabs {
        return Err(Error::MixedIndent);
    }

    if let Some(indent_style) = &extras.indent_style {
        let tabs_expected = matches!(indent_style, IndentStyle::Tabs);
        let spaces_expected = matches!(indent_style, IndentStyle::Spaces);

        if tabs_expected && !all_tabs {
            return Err(Error::SpaceIndent);
        }

        if spaces_expected && !all_spaces {
            return Err(Error::TabIndent);
        }

        Ok(if tabs_expected {
            IndentStyle::Tabs
        } else {
            IndentStyle::Spaces
        })
    } else {
        if all_tabs {
            return Ok(IndentStyle::Tabs);
        }

        if all_spaces {
            return Ok(IndentStyle::Spaces);
        }

        unreachable!();
    }
}
