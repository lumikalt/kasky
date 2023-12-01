use kasky::{parser::tokenize, fetch_file};

fn main() {
    tokenize(fetch_file("./example.ky".to_string()));
}
