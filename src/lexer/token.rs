use std::fmt::Display;

#[derive(Debug, PartialEq, Clone, Hash, Eq)]
pub struct Token {
    pub token_type: TokenType,
    pub literal: String,
}

impl Token {
    pub fn new(token_type: TokenType, literal: String) -> Self {
        Self { token_type, literal }
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Token({:?}, {})", self.token_type, self.literal)
    }
}

#[macro_export]
macro_rules! token {
    ($token_type:ident, $literal:expr) => {
        Token::new($token_type, $literal.to_string())
    };
}

#[derive(Debug, PartialEq, Clone, Hash, Eq)]
pub enum TokenType {
    Let,
    Ident,
    Int,
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

impl Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let token = match self {
            TokenType::Let => "let",
            TokenType::Ident => "IDENT",
            TokenType::Int => "INT",
            TokenType::Assign => "=",
            TokenType::Plus => "+",
            TokenType::Lparen => "(",
            TokenType::Rparen => ")",
            TokenType::Lbrace => "{",
            TokenType::Rbrace => "}",
            TokenType::Comma => ",",
            TokenType::Semicolon => ";",
            TokenType::Eof => "EOF",
            TokenType::Illegal => "ILLEGAL",
            TokenType::Function => "fn",
            TokenType::Bang => "!",
            TokenType::Dash => "-",
            TokenType::GreaterThan => ">",
            TokenType::LesserThan => "<",
            TokenType::Equals => "==",
            TokenType::If => "if",
            TokenType::Else => "else",
            TokenType::Asterisk => "*",
            TokenType::ForwardSlash => "/",
            TokenType::NotEqual => "!=",
            TokenType::False => "false",
            TokenType::True => "true",
            TokenType::Return => "return",
        };
        write!(f, "{}", token)
    }
}