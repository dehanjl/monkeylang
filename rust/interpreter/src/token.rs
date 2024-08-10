#[derive(PartialEq, Debug)]
pub enum Token {
    ILLEGAL,
    EOF,
    // Identifiers + literals

    // Operators
    ASSIGN,
    PLUS,

    // Delimiters
    COMMA,
    SEMICOLON,

    LPAREN,
    RPAREN,
    LBRACE,
    RBRACE,
    // Keywords
}
