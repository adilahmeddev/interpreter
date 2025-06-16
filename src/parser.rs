use crate::{lexer::Lexer, types::Token};

pub(crate) struct Parser {
    cur_token: Token,
    next_token: Token,
    l: Lexer,
}

impl Parser {
    pub(crate) fn new(mut l: Lexer) -> Self {
        let cur_token = l.next_token();
        let next_token = l.next_token();
        Self {
            cur_token,
            next_token,
            l: l,
        }
    }

    pub(crate) fn view_tokens(self) {
        println!(
            "cur: {:?}   -   next: {:?}",
            self.cur_token, self.next_token
        );
    }
}
