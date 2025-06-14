use crate::{lexer::Lexer, types::Token};
use std::io::Write;

pub fn start(input: std::io::Stdin, mut output: std::io::Stdout) {
    for line in input.lines() {
        if let Ok(l) = line {
            let mut lexer = Lexer {
                input: l,
                position: 0,
                read_position: 0,
                ch: '0',
            };
            loop {
                let token = lexer.next_token();
                if token == Token::Eof {
                    break;
                }
                if let Err(err) = output.write_all(format!("{:?}\n", token).as_bytes()) {
                    panic!("{err}");
                }
            }
        }
    }
}
