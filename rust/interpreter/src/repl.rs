use std::io::{self, Write};

use crate::lexer;

pub fn start() {
    loop {
        print!(">> ");
        let _ = io::stdout().flush();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let lexer = lexer::Lexer::new(&input);
        for token in lexer {
            println!("{:?}", token);
        }
    }
}
