use std::fmt::Display;

#[derive(Debug, PartialEq, Clone, Hash, Eq)]
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

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let token = match self {
            Token::Let => "let",
            Token::Ident(name) => name,
            Token::Int(value) => value,
            Token::Assign => "=",
            Token::Plus => "+",
            Token::Lparen => "(",
            Token::Rparen => ")",
            Token::Lbrace => "{",
            Token::Rbrace => "}",
            Token::Comma => ",",
            Token::Semicolon => ";",
            Token::Eof => "EOF",
            Token::Illegal => "ILLEGAL",
            Token::Function => "fn",
            Token::Bang => "!",
            Token::Dash => "-",
            Token::GreaterThan => ">",
            Token::LesserThan => "<",
            Token::Equals => "==",
            Token::If => "if",
            Token::Else => "else",
            Token::Asterisk => "*",
            Token::ForwardSlash => "/",
            Token::NotEqual => "!=",
            Token::False => "false",
            Token::True => "true",
            Token::Return => "return",
        };
        write!(f, "{}", token)
    }
}