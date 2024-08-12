#[derive(PartialEq, Debug)]
pub enum Token {
    ILLEGAL,
    EOF,
    // Identifiers + literals
    IDENT(String), // add, foobar, x, y, ...
    INT(String),   // 1343456

    // Operators
    ASSIGN,
    BANG,
    PLUS,
    MINUS,
    ASTERISK,
    SLASH,

    LT,
    GT,

    EQ,
    NOT_EQ,

    // Delimiters
    COMMA,
    SEMICOLON,

    LPAREN,
    RPAREN,
    LBRACE,
    RBRACE,
    // Keywords
    FUNCTION,
    LET,
    TRUE,
    FALSE,
    IF,
    ELSE,
    RETURN,
}

/// Acts as a lookup table for keywords
/// Returns the keyword if it exists, otherwise returns a `Token::IDENT`
pub fn lookup_ident(ident: &str) -> Token {
    match ident {
        "fn" => Token::FUNCTION,
        "let" => Token::LET,
        "true" => Token::TRUE,
        "false" => Token::FALSE,
        "if" => Token::IF,
        "else" => Token::ELSE,
        "return" => Token::RETURN,
        _ => Token::IDENT(ident.to_string()),
    }
}
