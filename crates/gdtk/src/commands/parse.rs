use owo_colors::OwoColorize;

use crate::display::print_error;

pub fn run(file: &String) -> anyhow::Result<()> {
    let content = std::fs::read_to_string(file)?;
    let lexed = gdtk_lexer::lex(&content);

    for lexeme in lexed {
        print_diag(file, &lexeme);
    }

    Ok(())
}

pub fn print_diag(file: &String, lexeme: &gdtk_lexer::Lexeme) {
    match lexeme {
        Ok((token, _)) => println!("{:?}", token),
        Err((err, span)) => {
            print_error(err.to_string());
            eprintln!("{} {}:{:?}", "-->".cyan(), file, span);
        },
    }
}
