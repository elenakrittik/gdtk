use combine::{easy::Stream, EasyParser};

use crate::display::{action, path};

pub fn run(file: &String) -> anyhow::Result<()> {
    println!("{} {}", action("Parsing"), path(file));

    let content = std::fs::read_to_string(file)?;

    println!("File contents: \n{}", content);

    let result = crate::parser::parser().easy_parse(Stream(&*content));

    let (ast, _) = match result {
        Ok(val) => val,
        Err(e) => {
            return {
                crate::display::display_ariadne_traceback(file, &content, e);
                Ok(())
            }
        }
    };

    println!("{:?}", ast);

    Ok(())
}
