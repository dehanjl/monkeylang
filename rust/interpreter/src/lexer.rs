use crate::token::{lookup_ident, Token};
use anyhow::Result;

pub struct Lexer {
    input: Vec<u8>,
    position: usize,      // current position in input (points to current char)
    read_position: usize, // current reading position in input (after current char)
    ch: char,             // current char under examination
}

impl Lexer {
    pub fn new(input: &str) -> Lexer {
        let mut l = Lexer {
            input: input.to_string().into_bytes(),
            position: 0,
            read_position: 0,
            ch: '\0',
        };
        l.read_char();
        l
    }

    fn read_char(&mut self) {
        // TODO: see if I can make this not use to_owned
        self.ch = self.input.get(self.read_position).unwrap_or(&0).to_owned() as char;

        self.position = self.read_position;
        self.read_position += 1;
    }

    fn peek_char(&self) -> char {
        // TODO: see if I can make this not use to_owned
        self.input.get(self.read_position).unwrap_or(&0).to_owned() as char
    }

    fn skip_whitespace(&mut self) {
        while self.ch.is_whitespace() {
            self.read_char();
        }
    }

    fn read_identifier(&mut self) -> Result<String> {
        let position = self.position;
        while self.ch.is_ascii_alphabetic() || self.ch == '_' {
            self.read_char();
        }
        
        Ok(String::from_utf8(
            self.input[position..self.position].to_vec(),
        )?)
    }

    fn read_number(&mut self) -> Result<String> {
        let position = self.position;
        while self.ch.is_ascii_digit() {
            self.read_char();
        }

        Ok(String::from_utf8(
            self.input[position..self.position].to_vec(),
        )?)
    }

    fn next_token(&mut self) -> Option<Token> {
        self.skip_whitespace();
        let tok = match (self.ch, self.peek_char()) {
            ('=', '=') => {
                self.read_char();
                Token::Equal
            }
            ('=', _) => Token::Assign,
            ('!', '=') => {
                self.read_char();
                Token::NotEqual
            }
            ('!', _) => Token::Bang,
            ('+', _) => Token::Plus,
            ('-', _) => Token::Minus,
            ('*', _) => Token::Asterisk,
            ('/', _) => Token::Slash,
            ('<', _) => Token::Less,
            ('>', _) => Token::Greater,
            (',', _) => Token::Comma,
            (';', _) => Token::Semicolon,
            ('(', _) => Token::LeftParen,
            (')', _) => Token::RightParen,
            ('{', _) => Token::LeftBrace,
            ('}', _) => Token::RightBrace,
            ('0'..='9', _) => {
                return Some(if let Ok(num) = self.read_number() {
                    Token::Int(num)
                } else {
                    Token::Illegal
                });
            }
            ('a'..='z', _) | ('A'..='Z', _) | ('_', _) => {
                return Some(if let Ok(ident) = self.read_identifier() {
                    lookup_ident(ident.as_str())
                } else {
                    Token::Illegal
                });
            }
            ('\0', _) => return None,
            _ => Token::Illegal,
        };
        self.read_char();
        Some(tok)
    }
}

impl Iterator for Lexer {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_token()
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_next_token_simple() {
        let input = "=+(){},;";
        let tests = [
            Token::Assign,
            Token::Plus,
            Token::LeftParen,
            Token::RightParen,
            Token::LeftBrace,
            Token::RightBrace,
            Token::Comma,
            Token::Semicolon,
        ];
        let mut l = Lexer::new(input);

        for (i, tt) in tests.iter().enumerate() {
            let tok = l.next().unwrap();
            assert_eq!(
                tok, *tt,
                "tests[{i}] - tokentype wrong. expected={tt:?}, got={tok:?}"
            );
        }

        assert_eq!(l.next(), None, "expected next to be None");
    }

    #[test]
    fn test_next_token() {
        let input = r#"let five = 5;
let ten = 10;

let add = fn(x, y) {
  x + y;
};

let result = add(five, ten);
"#;
        let tests = [
            Token::Let,
            Token::Ident("five".to_owned()),
            Token::Assign,
            Token::Int("5".to_owned()),
            Token::Semicolon,
            Token::Let,
            Token::Ident("ten".to_owned()),
            Token::Assign,
            Token::Int("10".to_owned()),
            Token::Semicolon,
            Token::Let,
            Token::Ident("add".to_owned()),
            Token::Assign,
            Token::Function,
            Token::LeftParen,
            Token::Ident("x".to_owned()),
            Token::Comma,
            Token::Ident("y".to_owned()),
            Token::RightParen,
            Token::LeftBrace,
            Token::Ident("x".to_owned()),
            Token::Plus,
            Token::Ident("y".to_owned()),
            Token::Semicolon,
            Token::RightBrace,
            Token::Semicolon,
            Token::Let,
            Token::Ident("result".to_owned()),
            Token::Assign,
            Token::Ident("add".to_owned()),
            Token::LeftParen,
            Token::Ident("five".to_owned()),
            Token::Comma,
            Token::Ident("ten".to_owned()),
            Token::RightParen,
            Token::Semicolon,
        ];

        let mut l = Lexer::new(input);
        for (i, tt) in tests.iter().enumerate() {
            let tok = l.next().unwrap();
            assert_eq!(
                tok, *tt,
                "tests[{i}] - wrong token found. expected={tt:?}, got={tok:?}"
            );
        }

        assert_eq!(l.next(), None, "expected next to be None");
    }

    #[test]
    fn test_next_token_full() {
        let input = r#"let five = 5;
let ten = 10;

let add = fn(x, y) {
  x + y;
};

let result = add(five, ten);
!-/*5;
5 < 10 > 5;

if (5 < 10) {
	return true;
} else {
	return false;
}

10 == 10;
10 != 9;
"#;
        let tests = [
            Token::Let,
            Token::Ident("five".to_owned()),
            Token::Assign,
            Token::Int("5".to_owned()),
            Token::Semicolon,
            Token::Let,
            Token::Ident("ten".to_owned()),
            Token::Assign,
            Token::Int("10".to_owned()),
            Token::Semicolon,
            Token::Let,
            Token::Ident("add".to_owned()),
            Token::Assign,
            Token::Function,
            Token::LeftParen,
            Token::Ident("x".to_owned()),
            Token::Comma,
            Token::Ident("y".to_owned()),
            Token::RightParen,
            Token::LeftBrace,
            Token::Ident("x".to_owned()),
            Token::Plus,
            Token::Ident("y".to_owned()),
            Token::Semicolon,
            Token::RightBrace,
            Token::Semicolon,
            Token::Let,
            Token::Ident("result".to_owned()),
            Token::Assign,
            Token::Ident("add".to_owned()),
            Token::LeftParen,
            Token::Ident("five".to_owned()),
            Token::Comma,
            Token::Ident("ten".to_owned()),
            Token::RightParen,
            Token::Semicolon,
            Token::Bang,
            Token::Minus,
            Token::Slash,
            Token::Asterisk,
            Token::Int("5".to_owned()),
            Token::Semicolon,
            Token::Int("5".to_owned()),
            Token::Less,
            Token::Int("10".to_owned()),
            Token::Greater,
            Token::Int("5".to_owned()),
            Token::Semicolon,
            Token::If,
            Token::LeftParen,
            Token::Int("5".to_owned()),
            Token::Less,
            Token::Int("10".to_owned()),
            Token::RightParen,
            Token::LeftBrace,
            Token::Return,
            Token::True,
            Token::Semicolon,
            Token::RightBrace,
            Token::Else,
            Token::LeftBrace,
            Token::Return,
            Token::False,
            Token::Semicolon,
            Token::RightBrace,
            Token::Int("10".to_owned()),
            Token::Equal,
            Token::Int("10".to_owned()),
            Token::Semicolon,
            Token::Int("10".to_owned()),
            Token::NotEqual,
            Token::Int("9".to_owned()),
            Token::Semicolon,
        ];
        let mut l = Lexer::new(input);

        for (i, tt) in tests.iter().enumerate() {
            let tok = l.next().unwrap();
            assert_eq!(
                tok, *tt,
                "tests[{i}] - tokentype wrong. expected={tt:?}, got={tok:?}"
            );
        }

        assert_eq!(l.next(), None, "expected next to be None");
    }
}
