use crate::lexer::token::Token;
use crate::lexer::token::Token::*;

pub mod token;

pub struct Lexer {
    input: Vec<u8>,
    current_position: usize,
    read_position: usize,
    ch: u8,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        let mut lex = Self {
            input: input.into_bytes(),
            current_position: 0,
            read_position: 0,
            ch: 0,
        };
        lex.read_char();
        lex
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();
        let tok: Token = match self.ch {
            b'=' => {
                if self.lookahead() == b'=' {
                    self.read_char();
                    Equals
                } else {
                    Assign
                }
            },
            b'+' => Plus,
            b'-' => Dash,
            b'*' => Asterisk,
            b'(' => Lparen,
            b')' => Rparen,
            b'{' => Lbrace,
            b'}' => Rbrace,
            b',' => Comma,
            b';' => Semicolon,
            b'!' => {
                if self.lookahead() == b'=' {
                    self.read_char();
                    NotEqual
                } else {
                    Bang
                }
            },
            b'/' => ForwardSlash,
            b'<' => LesserThan,
            b'>' => GreaterThan,
            b'a'..=b'z' => {
                let id = self.read_ident();
                return match id.as_str() {
                    "let" => Let,
                    "fn" => Function,
                    "else" => Else,
                    "if" => If,
                    "true" => True,
                    "false" => False,
                    "return" => Return,
                    _ => Ident(id),
                }
            },
            b'0'..=b'9' => return Int(self.read_num()),
            0 => Eof,
            _ => Illegal,
        };
        self.read_char();
        tok
    }

    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = 0;
        } else {
            self.ch = self.input[self.read_position];
        }

        self.current_position = self.read_position;
        self.read_position += 1;
    }
    
    fn read_ident(&mut self) -> String {
        let prev_position = self.current_position;
        while self.ch.is_ascii_alphabetic() || self.ch == b'_' {
            self.read_char();
        }
        String::from_utf8_lossy(&self.input[prev_position..self.current_position]).to_string()
    }

    fn read_num(&mut self) -> String {
        let prev_position = self.current_position;
        while self.ch.is_ascii_digit() {
            self.read_char();
        }
        String::from_utf8_lossy(&self.input[prev_position..self.current_position]).to_string()
    }
    
    fn skip_whitespace(&mut self) {
        while self.ch.is_ascii_whitespace() {
            self.read_char();
        }
    }
    
    fn lookahead(&mut self) -> u8 {
        return if self.read_position >= self.input.len() {
            0
        } else {
            self.input[self.read_position]
        }
    }
}

impl Iterator for Lexer {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        let tok = self.next_token();
        if tok == Eof {
            None
        } else {
            Some(tok)
        }
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize_simple() {
        let input = "=+(){},;";
        let expected = vec![
            Assign,
            Plus,
            Lparen,
            Rparen,
            Lbrace,
            Rbrace,
            Comma,
            Semicolon
        ];
        let mut lex = Lexer::new(input.to_string());
        for tok in expected.iter() {
            assert_eq!(&lex.next_token(), tok)
        }
    }

    #[test]
    fn test_tokenize_complex() {
        let input = r#"let five = 5;
            let ten = 10;

            let add = fn(x,y) {
                x+y;
            };

            let result = add(five,ten);
            !-/*5;
            5 < 10 > 5;
            "#;
        
        let expected = vec![
            Let,
            Ident(String::from("five")),
            Assign,
            Int(String::from("5")),
            Semicolon,
            Let,
            Ident(String::from("ten")),
            Assign,
            Int(String::from("10")),
            Semicolon,
            Let,
            Ident(String::from("add")),
            Assign,
            Function,
            Lparen,
            Ident(String::from("x")),
            Comma,
            Ident(String::from("y")),
            Rparen,
            Lbrace,
            Ident(String::from("x")),
            Plus,
            Ident(String::from("y")),
            Semicolon,
            Rbrace,
            Semicolon,
            Let,
            Ident(String::from("result")),
            Assign,
            Ident(String::from("add")),
            Lparen,
            Ident(String::from("five")),
            Comma,
            Ident(String::from("ten")),
            Rparen,
            Semicolon,
            Bang,
            Dash,
            ForwardSlash,
            Asterisk,
            Int(String::from("5")),
            Semicolon,
            Int(String::from("5")),
            LesserThan,
            Int(String::from("10")),
            GreaterThan,
            Int(String::from("5")),
            Semicolon,
            Eof,
        ];
        let mut lex = Lexer::new(input.to_string());
        for (tok, i) in expected.iter().zip(0..expected.len()) {
            let got = lex.next_token();
            println!("expected: {:?}, got: {:?} at: {}", tok, got, i);
            assert_eq!(&got, tok)
        }
    }
}