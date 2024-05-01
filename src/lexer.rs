use crate::lexer::token::Token;
use crate::lexer::token::TokenType::*;
use crate::token;

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
                    token!(Equals, "==")
                } else {
                    token!(Assign, "=")
                }
            },
            b'+' => token!(Plus, "+"),
            b'-' => token!(Dash, "-"),
            b'*' => token!(Asterisk, "*"),
            b'(' => token!(Lparen, "("),
            b')' => token!(Rparen, ")"),
            b'{' => token!(Lbrace, "{"),
            b'}' => token!(Rbrace, "}"),
            b',' => token!(Comma, ","),
            b';' => token!(Semicolon, ";"),
            b'!' => {
                if self.lookahead() == b'=' {
                    self.read_char();
                    token!(NotEqual, "!=")
                } else {
                    token!(Bang, "!")
                }
            },
            b'/' => token!(ForwardSlash, "/"),
            b'<' => token!(LesserThan, "<"),
            b'>' => token!(GreaterThan, ">"),
            b'a'..=b'z' => {
                let id = self.read_ident();
                return match id.as_str() {
                    "let" => token!(Let, "let"),
                    "fn" => token!(Function, "fn"),
                    "else" => token!(Else, "else"),
                    "if" => token!(If, "if"),
                    "true" => token!(True, "true"),
                    "false" => token!(False, "false"),
                    "return" => token!(Return, "return"),
                    _ => token!(Ident, id),
                }
            },
            b'0'..=b'9' => return token!(Int, self.read_num()),
            0 => token!(Eof, ""),
            _ => token!(Illegal, ""),
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
        if tok.token_type == Eof {
            None
        } else {
            Some(tok)
        }
    }
}



#[cfg(test)]
mod tests {
    use crate::lexer::token::TokenType::Assign;
    use crate::token;
    use super::*;

    #[test]
    fn test_tokenize_simple() {
        let input = "=+(){},;";
        let expected = vec![
            token!(Assign, "="),
            token!(Plus, "+"),
            token!(Lparen, "("),
            token!(Rparen, ")"),
            token!(Lbrace, "{"),
            token!(Rbrace, "}"),
            token!(Comma, ","),
            token!(Semicolon, ";"),
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
            token!(Let, "let"),
            token!(Ident, "five"),
            token!(Assign, "="),
            token!(Int, "5"),
            token!(Semicolon, ";"),
            token!(Let, "let"),
            token!(Ident, "ten"),
            token!(Assign, "="),
            token!(Int, "10"),
            token!(Semicolon, ";"),
            token!(Let, "let"),
            token!(Ident, "add"),
            token!(Assign, "="),
            token!(Function, "fn"),
            token!(Lparen, "("),
            token!(Ident, "x"),
            token!(Comma, ","),
            token!(Ident, "y"),
            token!(Rparen, ")"),
            token!(Lbrace, "{"),
            token!(Ident, "x"),
            token!(Plus, "+"),
            token!(Ident, "y"),
            token!(Semicolon, ";"),
            token!(Rbrace, "}"),
            token!(Semicolon, ";"),
            token!(Let, "let"),
            token!(Ident, "result"),
            token!(Assign, "="),
            token!(Ident, "add"),
            token!(Lparen, "("),
            token!(Ident, "five"),
            token!(Comma, ","),
            token!(Ident, "ten"),
            token!(Rparen, ")"),
            token!(Semicolon, ";"),
            token!(Bang, "!"),
            token!(Dash, "-"),
            token!(ForwardSlash, "/"),
            token!(Asterisk, "*"),
            token!(Int, "5"),
            token!(Semicolon, ";"),
            token!(Int, "5"),
            token!(LesserThan, "<"),
            token!(Int, "10"),
            token!(GreaterThan, ">"),
            token!(Int, "5"),
            token!(Semicolon, ";"),
        ];
        let mut lex = Lexer::new(input.to_string());
        for (tok, i) in expected.iter().zip(0..expected.len()) {
            let got = lex.next_token();
            //println!("expected: {:?}, got: {:?} at: {}", tok, got, i);
            assert_eq!(&got, tok)
        }
    }
}