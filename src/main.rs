use std::io::{stdin, stdout};

mod parser;
mod repl;
mod types;

fn main() {
    // repl::start(stdin(), stdout());
    let p = parser::Parser::new(lexer::Lexer {
        input: "let 1".to_string(),
        position: 0,
        read_position: 0,
        ch: '0',
    });
    p.view_tokens();
}

mod lexer;
