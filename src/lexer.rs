use types::Token;

use crate::types;

pub(crate) struct Lexer {
    pub(crate) input: String,
    pub(crate) position: usize,
    pub(crate) read_position: usize,
    pub(crate) ch: char,
}

impl Lexer {
    fn read(&mut self) {
        if self.position >= self.input.len() {
            return;
        }
        self.ch = *self
            .input
            .chars()
            .collect::<Vec<char>>()
            .get(self.position)
            .unwrap_or(&'\0');
    }

    fn peek(&mut self) -> char {
        *self
            .input
            .chars()
            .collect::<Vec<char>>()
            .get(self.read_position)
            .unwrap_or(&'\0')
    }

    pub(crate) fn next_token(&mut self) -> Token {
        if self.position == self.input.len() {
            return Token::Eof;
        }
        self.read();
        while self.ch == ' ' {
            self.position += 1;
            self.read();
        }
        let tok = match self.ch {
            '(' => Token::LParen,
            ')' => Token::RParen,
            '{' => Token::LBrace,
            '}' => Token::RBrace,
            '+' => Token::Plus,
            '-' => Token::Minus,
            ',' => Token::Comma,
            '*' => Token::Asterisk,
            '!' => Token::Bang,
            '/' => Token::Slash,
            '<' => Token::Lt,
            '>' => Token::Gt,
            ':' => Token::Colon,
            '[' => Token::LBracket,
            ']' => Token::RBracket,
            ';' => Token::Semicolon,
            '0'..='9' => {
                let (start, end) = self.read_string('0', '9');
                Token::Int(self.input[start..end].to_string())
            }
            '=' => Token::Eq,
            'a'..='z' | 'A'..='Z' => {
                let (start, end) = self.read_string_2('a', 'z', 'A', 'Z');
                Token::get_keyword(&self.input[start..end])
            }
            _ => Token::Illegal,
        };
        self.position += 1;
        tok
    }

    fn read_string_2(&mut self, a: char, b: char, c: char, d: char) -> (usize, usize) {
        self.read_position = self.position;

        let start = self.position;
        let mut end = self.position + 1;
        while self.position < self.input.len() {
            let next = self.peek();
            if a <= next && b >= next || c <= next && d >= next {
                end += 1
            } else {
                self.position = self.read_position - 1;
                break;
            }
            self.read_position = end;
        }
        (start, end)
    }
    fn read_string(&mut self, a: char, b: char) -> (usize, usize) {
        self.read_position = self.position;

        let start = self.position;
        let mut end = self.position + 1;
        while self.position < self.input.len() {
            let next = self.peek();
            if a <= next && b >= next {
                end += 1
            } else {
                self.position = self.read_position - 1;
                break;
            }
            self.read_position = end;
        }
        (start, end)
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;

    #[test]
    fn read_char() {
        let mut lexer = Lexer {
            input: String::from("999+(){},;hello let Hello"),
            position: 0,
            read_position: 0,
            ch: '0',
        };

        assert_eq!(lexer.next_token(), Token::Int(String::from("999")));
        assert_eq!(lexer.next_token(), Token::Plus);
        assert_eq!(lexer.next_token(), Token::LParen);
        assert_eq!(lexer.next_token(), Token::RParen);
        assert_eq!(lexer.next_token(), Token::LBrace);
        assert_eq!(lexer.next_token(), Token::RBrace);
        assert_eq!(lexer.next_token(), Token::Comma);
        assert_eq!(lexer.next_token(), Token::Semicolon);
        assert_eq!(lexer.next_token(), Token::Ident("hello".to_string()));
        assert_eq!(lexer.next_token(), Token::Let);
        assert_eq!(lexer.next_token(), Token::Ident("Hello".to_string()));
    }
}
