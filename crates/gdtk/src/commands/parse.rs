use owo_colors::OwoColorize;

use crate::display::{print_error, ACTION, THING};

pub fn run(file: &String) -> anyhow::Result<()> {
    println!("{} {}", "Parsing".style(*ACTION), file.style(*THING));

    let content = std::fs::read_to_string(file)?;
    let lexed = match gdtk_lexer::lex(&content) {
        Ok(v) => v,
        Err(e) => {
            print_error(e.to_string());
            println!("{} {}:{:?}", "-->".cyan(), file, e.1);
            return Ok(());
        }
    };

    for lexeme in lexed {
        println!("{:?}", lexeme);
    }

    Ok(())
}
