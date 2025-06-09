use std::io::{stdin, stdout};

mod repl;
mod types;

fn main() {
    repl::start(stdin(), stdout());
}

mod lexer;
