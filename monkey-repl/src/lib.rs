use monkey_lexer::{Lexer, Token};

pub struct Repl {
    lexer: Lexer,
}

impl Repl {
    pub fn new() -> Self {
        let lexer = Lexer::new();
        Repl { lexer }
    }

    pub fn evaluate(&self, input: &str) -> Vec<Token> {
        self.lexer.parse(input)
    }
}
