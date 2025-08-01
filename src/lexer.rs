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
        if self.position >= self.input.len() {
            return Token::Eof;
        }
        self.read();
        while self.ch == ' ' {
            self.position += 1;
            self.read();
        }
        let tok = match self.ch {
            '(' => Token::LParen,
            '"' => {
                let (start, end) = self.read_full_string();
                Token::String(self.input[start..end].to_string())
            }
            ')' => Token::RParen,
            '{' => Token::LBrace,
            '}' => Token::RBrace,
            '+' => Token::Plus,
            '-' => Token::Minus,
            ',' => Token::Comma,
            '*' => Token::Asterisk,
            '!' => {
                let (start, end) = self.read_string('!', '=');
                if end - start == 2 {
                    Token::NotEq
                } else {
                    Token::Bang
                }
            }
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
            '=' => {
                let (start, end) = self.read_string('=', '=');
                if end - start == 2 {
                    Token::Eq
                } else {
                    Token::Assign
                }
            }
            'a'..='z' | 'A'..='Z' => {
                let (start, end) = self.read_string('a', 'z');
                Token::get_keyword(&self.input[start..end])
            }
            _ => Token::Illegal,
        };
        self.position += 1;
        tok
    }

    fn read_full_string(&mut self) -> (usize, usize) {
        self.read_position = self.position;

        let start = self.position + 1;
        let mut end = self.position + 2;
        while self.position < self.input.len()
            && end < self.input.len()
            && self.input.chars().collect::<Vec<char>>()[end] != '"'
        {
            end += 1;
            self.position = end;
        }
        self.read_position = self.position;
        (start, end)
    }

    fn read_string(&mut self, a: char, b: char) -> (usize, usize) {
        self.read_position = self.position;
        let mut first = a;
        let mut second = b;
        if a.is_alphabetic() {
            if let Some(lower) = a.to_lowercase().next() {
                first = lower;
            }
        }
        if b.is_alphabetic() {
            if let Some(lower) = b.to_lowercase().next() {
                second = lower;
            }
        }
        let start = self.position;
        let mut end = self.position;
        while self.position < self.input.len() {
            let mut next = self.peek();
            next = next.to_lowercase().next().unwrap_or(next);
            if first <= next && second >= next {
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
    fn assign() {
        let mut lexer = Lexer {
            input: String::from("="),
            position: 0,
            read_position: 0,
            ch: '0',
        };

        assert_eq!(lexer.next_token(), Token::Assign);
    }

    #[test]
    fn eq() {
        let mut lexer = Lexer {
            input: String::from("=="),
            position: 0,
            read_position: 0,
            ch: '0',
        };

        assert_eq!(lexer.next_token(), Token::Eq);
    }

    #[test]
    fn read_char() {
        let mut lexer = Lexer {
            input: String::from("999+ ! (){},;hello lEt = Hello == != \"string hello\""),
            position: 0,
            read_position: 0,
            ch: '0',
        };

        assert_eq!(lexer.next_token(), Token::Int(String::from("999")));
        assert_eq!(lexer.next_token(), Token::Plus);
        assert_eq!(lexer.next_token(), Token::Bang);
        assert_eq!(lexer.next_token(), Token::LParen);
        assert_eq!(lexer.next_token(), Token::RParen);
        assert_eq!(lexer.next_token(), Token::LBrace);
        assert_eq!(lexer.next_token(), Token::RBrace);
        assert_eq!(lexer.next_token(), Token::Comma);
        assert_eq!(lexer.next_token(), Token::Semicolon);
        assert_eq!(lexer.next_token(), Token::Ident("hello".to_string()));
        assert_eq!(lexer.next_token(), Token::Let);
        assert_eq!(lexer.next_token(), Token::Assign);
        assert_eq!(lexer.next_token(), Token::Ident("Hello".to_string()));
        assert_eq!(lexer.next_token(), Token::Eq);
        assert_eq!(lexer.next_token(), Token::NotEq);
        assert_eq!(
            lexer.next_token(),
            Token::String("string hello".to_string())
        );
    }
}
