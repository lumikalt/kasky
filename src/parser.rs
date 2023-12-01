pub struct Location {
    start: usize, // Offset from start of file.
    end: usize, // where the current token ends, relative to start.
    line: usize, // Pretty-printing.
    column: usize, // Pretty-printing.
}

enum Token {
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
    current: usize,
}
