use gdtk_parser::parse_file;
// use owo_colors::OwoColorize;

// use crate::display::print_error;

pub fn run(file: &String) -> anyhow::Result<()> {
    let mut i = 0;
    loop {
        i += 1;
        let content = std::fs::read_to_string(file)?;
        let lexed = gdtk_lexer::lex(&content);

        #[cfg(debug_assertions)]
        dbg!(&lexed);

        let parsed = parse_file(lexed.clone())?;

        #[cfg(debug_assertions)]
        dbg!(&parsed.body);

        if i >= 1 {
            break;
        }
    }

    // dbg!(&lexed.0);
    // dbg!(&lexed.1);

    // for lexeme in lexed {
    //     print_diag(file, &lexeme);
    // }

    Ok(())
}

// pub fn print_diag(file: &String, lexeme: &gdtk_lexer::Lexeme) {
//     match lexeme {
//         Ok((token, _)) => println!("{:?}", token),
//         Err((err, span)) => {
//             print_error(err.to_string());
//             eprintln!("{} {}:{:?}", "-->".cyan(), file, span);
//         },
//     }
// }
