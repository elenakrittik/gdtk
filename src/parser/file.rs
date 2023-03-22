use std::fs::File;
use std::io::{self, prelude::*, BufReader};

pub fn parse_file(path: String) -> Result<String, io::Error> {
    let fp = match File::open(path) {
        Ok(fp) => fp,
        Err(e) => return Err(e),
    };
    let reader = BufReader::new(fp);
    let mut content = String::new();

    let line_iter = reader.lines();

    for line_option in line_iter {
        let line = match line_option {
            Ok(ln) => ln,
            Err(_) => break,
        };
        parse_line(&mut content, &line);
    }

    Result::Ok(content)
}

fn parse_line(content: &mut String, line: &String) {
    let mut result = String::new();

    let mut chars = line.trim().chars();
    let first_char = match chars.next() {
        Some(chr) => chr,
        None => {
            content.push('\n');
            return;
        }
    };

    match first_char {
        '#' => {
            let stripped = line.trim().strip_prefix("#").unwrap().trim();
            let mut il = String::from("# ");
            il.push_str(stripped);
            result.push_str(&il);
        }
        _ => {}
    }

    content.push_str(&result);
    content.push('\n');
}
