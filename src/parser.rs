use rayon::prelude::*;

pub struct Location {
    start: usize,  // Offset from start of file.
    end: usize,    // where the current token ends, relative to start.
    line: usize,   // Pretty-printing.
    column: usize, // Pretty-printing.
}

pub enum Token {
    // Registers.
    PowerPin(String),
    XBus(String),

    // Values.
    Character(char),
    String(String),
    Number(i64),
    Boolean(bool),

    // Commands
    Definition(String),
    Call(String, Vec<String>),
}

struct Parser {
    tokens: Vec<Token>,
    loc: Location,
}

impl Parser {
    fn new(tokens: Vec<Token>) -> Parser {
        Parser {
            tokens: tokens,
            loc: Location {
                start: 0,
                end: 0,
                line: 0,
                column: 0,
            },
        }
    }
}

pub fn tokenize(input: String) -> Vec<Token> {
    let mut tokens = vec![];
    let input = input.to_lowercase();

    let lines: Vec<String> = input.par_lines().map(|line| {
        let mut chars = line.chars().peekable();
        let mut tokens = vec![];

        while let Some(c) = chars.next() {
            match c {
                ' ' => continue,
                'a'..='z' => {
                    let mut word = String::new();
                    word.push(c);

                    while let Some(&c) = chars.peek() {
                        if c.is_alphabetic() {
                            word.push(chars.next().unwrap());
                        } else {
                            break;
                        }
                    }

                    tokens.push(Token::Definition(word));
                },
                _ => {
                    tokens.push(Token::Character(c));
                },
            }
        }

        "uwu".to_string()
    }).collect();

    dbg!(lines);

    tokens
}
