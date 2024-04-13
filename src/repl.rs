use std::io;
use std::io::Write;
use crate::lexer::Lexer;

const PROMPT: &str = ">> ";
pub struct Repl {}
impl Repl {
    pub fn new() -> Self {
        Self {}
    }

    pub fn start(&self) {
        loop {
            let mut input = String::new();
            print!("{}", PROMPT);
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut input).unwrap();
            let lexer = Lexer::new(input);
            for token in lexer {
                println!("{:?}", token);
            }
        }
    }
}