#[derive(Debug, PartialEq)]
pub enum Token {
    Let,
    Ident(String),
    Int(String),
    Assign,
    Plus,
    Lparen,
    Rparen,
    Lbrace,
    Rbrace,
    Comma,
    Semicolon,
    Eof,
    Illegal,
    Function,
    Bang,
    Dash,
    GreaterThan,
    LesserThan,
    Equals,
    If,
    Else,
    Asterisk,
    ForwardSlash,
    NotEqual,
    False,
    True,
    Return,
}