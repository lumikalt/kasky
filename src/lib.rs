pub mod compiler;
pub mod error;
pub mod formatter;
pub mod parser;

use std::fs;

pub fn fetch_file(path: String) -> String {
    fs::read_to_string(path)
        .expect("[Critical]: File not found.")
        .to_string()
}
