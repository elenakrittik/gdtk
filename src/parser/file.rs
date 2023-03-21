use std::fs;
use std::io;

pub fn parse_file(file: String) -> Result<String, io::Error> {
    let content = match fs::read_to_string(file) {
        Ok(text) => text,
        Err(e) => return Err(e),
    };

    Result::Ok(content)
}
