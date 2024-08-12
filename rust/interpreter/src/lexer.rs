use crate::token::{lookup_ident, Token};
use anyhow::Result;

struct Lexer {
    input: Vec<u8>,
    position: usize,      // current position in input (points to current char)
    read_position: usize, // current reading position in input (after current char)
    ch: char,             // current char under examination
}

impl Lexer {
    fn new(input: &str) -> Lexer {
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
                Token::EQ
            }
            ('=', _) => Token::ASSIGN,
            ('!', '=') => {
                self.read_char();
                Token::NOT_EQ
            }
            ('!', _) => Token::BANG,
            ('+', _) => Token::PLUS,
            ('-', _) => Token::MINUS,
            ('*', _) => Token::ASTERISK,
            ('/', _) => Token::SLASH,
            ('<', _) => Token::LT,
            ('>', _) => Token::GT,
            (',', _) => Token::COMMA,
            (';', _) => Token::SEMICOLON,
            ('(', _) => Token::LPAREN,
            (')', _) => Token::RPAREN,
            ('{', _) => Token::LBRACE,
            ('}', _) => Token::RBRACE,
            ('0'..='9', _) => {
                return Some(if let Ok(num) = self.read_number() {
                    Token::INT(num)
                } else {
                    Token::ILLEGAL
                });
            }
            ('a'..='z', _) | ('A'..='Z', _) | ('_', _) => {
                return Some(if let Ok(ident) = self.read_identifier() {
                    lookup_ident(ident.as_str())
                } else {
                    Token::ILLEGAL
                });
            }
            ('\0', _) => return None,
            _ => Token::ILLEGAL,
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
            Token::ASSIGN,
            Token::PLUS,
            Token::LPAREN,
            Token::RPAREN,
            Token::LBRACE,
            Token::RBRACE,
            Token::COMMA,
            Token::SEMICOLON,
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
            Token::LET,
            Token::IDENT("five".to_owned()),
            Token::ASSIGN,
            Token::INT("5".to_owned()),
            Token::SEMICOLON,
            Token::LET,
            Token::IDENT("ten".to_owned()),
            Token::ASSIGN,
            Token::INT("10".to_owned()),
            Token::SEMICOLON,
            Token::LET,
            Token::IDENT("add".to_owned()),
            Token::ASSIGN,
            Token::FUNCTION,
            Token::LPAREN,
            Token::IDENT("x".to_owned()),
            Token::COMMA,
            Token::IDENT("y".to_owned()),
            Token::RPAREN,
            Token::LBRACE,
            Token::IDENT("x".to_owned()),
            Token::PLUS,
            Token::IDENT("y".to_owned()),
            Token::SEMICOLON,
            Token::RBRACE,
            Token::SEMICOLON,
            Token::LET,
            Token::IDENT("result".to_owned()),
            Token::ASSIGN,
            Token::IDENT("add".to_owned()),
            Token::LPAREN,
            Token::IDENT("five".to_owned()),
            Token::COMMA,
            Token::IDENT("ten".to_owned()),
            Token::RPAREN,
            Token::SEMICOLON,
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
            Token::LET,
            Token::IDENT("five".to_owned()),
            Token::ASSIGN,
            Token::INT("5".to_owned()),
            Token::SEMICOLON,
            Token::LET,
            Token::IDENT("ten".to_owned()),
            Token::ASSIGN,
            Token::INT("10".to_owned()),
            Token::SEMICOLON,
            Token::LET,
            Token::IDENT("add".to_owned()),
            Token::ASSIGN,
            Token::FUNCTION,
            Token::LPAREN,
            Token::IDENT("x".to_owned()),
            Token::COMMA,
            Token::IDENT("y".to_owned()),
            Token::RPAREN,
            Token::LBRACE,
            Token::IDENT("x".to_owned()),
            Token::PLUS,
            Token::IDENT("y".to_owned()),
            Token::SEMICOLON,
            Token::RBRACE,
            Token::SEMICOLON,
            Token::LET,
            Token::IDENT("result".to_owned()),
            Token::ASSIGN,
            Token::IDENT("add".to_owned()),
            Token::LPAREN,
            Token::IDENT("five".to_owned()),
            Token::COMMA,
            Token::IDENT("ten".to_owned()),
            Token::RPAREN,
            Token::SEMICOLON,
            Token::BANG,
            Token::MINUS,
            Token::SLASH,
            Token::ASTERISK,
            Token::INT("5".to_owned()),
            Token::SEMICOLON,
            Token::INT("5".to_owned()),
            Token::LT,
            Token::INT("10".to_owned()),
            Token::GT,
            Token::INT("5".to_owned()),
            Token::SEMICOLON,
            Token::IF,
            Token::LPAREN,
            Token::INT("5".to_owned()),
            Token::LT,
            Token::INT("10".to_owned()),
            Token::RPAREN,
            Token::LBRACE,
            Token::RETURN,
            Token::TRUE,
            Token::SEMICOLON,
            Token::RBRACE,
            Token::ELSE,
            Token::LBRACE,
            Token::RETURN,
            Token::FALSE,
            Token::SEMICOLON,
            Token::RBRACE,
            Token::INT("10".to_owned()),
            Token::EQ,
            Token::INT("10".to_owned()),
            Token::SEMICOLON,
            Token::INT("10".to_owned()),
            Token::NOT_EQ,
            Token::INT("9".to_owned()),
            Token::SEMICOLON,
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
