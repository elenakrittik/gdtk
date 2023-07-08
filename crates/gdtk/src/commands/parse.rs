use crate::display::{action, path};

pub fn run(file: &String) -> anyhow::Result<()> {
    println!("{} {}", action("Parsing"), path(file));

    let mut content = std::fs::read_to_string(file)?;

    println!("File contents: \n{}", content);

    let result = crate::parser::parse(&mut content);

    let ast = match result {
        Ok(val) => val,
        Err(e) => {
            return {
                // crate::display::display_ariadne_traceback(file, &content, e);
                eprintln!("{:#?}", e);
                Ok(())
            };
        }
    };

    println!("{:?}", ast);

    Ok(())
}
