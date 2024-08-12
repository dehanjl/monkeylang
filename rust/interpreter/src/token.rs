#[derive(PartialEq, Debug)]
pub enum Token {
    Illegal,

    // Identifiers + literals
    Ident(String), // add, foobar, x, y, ...
    Int(String),   // 1343456

    // Operators
    Assign,
    Bang,
    Plus,
    Minus,
    Asterisk,
    Slash,

    Less,
    Greater,

    Equal,
    NotEqual,

    // Delimiters
    Comma,
    Semicolon,

    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    // Keywords
    Function,
    Let,
    True,
    False,
    If,
    Else,
    Return,
}

/// Acts as a lookup table for keywords
/// Returns the keyword if it exists, otherwise returns a `Token::IDENT`
pub fn lookup_ident(ident: &str) -> Token {
    match ident {
        "fn" => Token::Function,
        "let" => Token::Let,
        "true" => Token::True,
        "false" => Token::False,
        "if" => Token::If,
        "else" => Token::Else,
        "return" => Token::Return,
        _ => Token::Ident(ident.to_string()),
    }
}
