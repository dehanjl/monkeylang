use std::str::Chars;

use crate::token::Token;

struct Lexer<'a> {
    input: String,
    position: usize,      // current position in input (points to current char)
    read_position: usize, // current reading position in input (after current char)
    ch: char,             // current char under examination
    chars: Chars<'a>,
}

impl Lexer<'_> {
    fn new(input: &str) -> Lexer<'_> {
        let mut l = Lexer {
            input: input.to_string(),
            position: 0,
            read_position: 0,
            ch: '\0',
            chars: input.chars(),
        };
        l.read_char();
        l
    }

    fn read_char(&mut self) {
        self.ch = self.chars.next().unwrap_or('\0');

        self.position = self.read_position;
        self.read_position += 1;
    }

    fn peek_char(&self) -> char {
        self.chars
            .clone()
            .peekable()
            .peek()
            .cloned()
            .unwrap_or('\0')
    }

    fn skip_whitespace(&mut self) {
        while self.ch.is_whitespace() {
            self.read_char();
        }
    }

    fn read_identifier(&mut self) -> String {
        let position = self.position;
        while self.ch.is_ascii_alphabetic() || self.ch == '_' {
            self.read_char();
        }
        self.input[position..self.position].to_string()
    }

    fn read_number(&mut self) -> String {
        let position = self.position;
        while self.ch.is_ascii_digit() {
            self.read_char();
        }
        self.input[position..self.position].to_string()
    }

    fn next_token(&mut self) -> Option<Token> {
        let tok = match self.ch {
            '=' => Token::ASSIGN,
            '+' => Token::PLUS,
            '(' => Token::LPAREN,
            ')' => Token::RPAREN,
            '{' => Token::LBRACE,
            '}' => Token::RBRACE,
            ',' => Token::COMMA,
            ';' => Token::SEMICOLON,
            '\0' => return None,
            _ => Token::ILLEGAL,
        };
        self.read_char();
        Some(tok)
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
            let tok = l.next_token().unwrap();
            assert_eq!(
                tok, *tt,
                "tests[{i}] - tokentype wrong. expected={tt:?}, got={tok:?}"
            );
        }
    }
}
