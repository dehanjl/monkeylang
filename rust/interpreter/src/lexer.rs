use crate::token::Token;

struct Lexer {
    input: String,
    position: usize,
    read_position: usize,
    ch: char,
}

impl Lexer {
    fn new(input: String) -> Lexer {
        let mut l = Lexer {
            input,
            position: 0,
            read_position: 0,
            ch: '\0',
        };
        l.read_char();
        l
    }

    fn read_char(&mut self) {
        if let Some(ch) = self.input.chars().nth(self.read_position) {
            self.ch = ch;
        } else {
            self.ch = '\0';
        }

        self.position = self.read_position;
        self.read_position += 1;
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
        let mut l = Lexer::new(input.to_string());

        for (i, tt) in tests.iter().enumerate() {
            let tok = l.next_token().unwrap();
            assert_eq!(
                tok, *tt,
                "tests[{i}] - tokentype wrong. expected={tt:?}, got={tok:?}"
            );
        }
    }
}
