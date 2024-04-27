use crate::ast::{Expression, Identifier, LetStatement, Literal, Program, Statement};
use crate::lexer::Lexer;
use crate::lexer::token::Token;

pub struct Parser {
    lexer: Lexer,
    current_token: Token,
    peek_token: Token,
}

impl Parser {
    pub fn new(lexer: Lexer) -> Self {
        let mut parser = Self {
            lexer,
            current_token: Token::Illegal,
            peek_token: Token::Illegal,
        };
        parser.next_token();
        parser.next_token();
        parser
    }

    pub fn next_token(&mut self) {
        self.current_token = self.peek_token.clone();
        self.peek_token = self.lexer.next_token();
    }
    
    pub fn parse_program(&mut self) -> Result<Program, String> {
        let mut program = Program { statements: vec![] };
        while self.current_token != Token::Eof {
            let statement = self.parse_statement()?;
            program.statements.push(statement);
            self.next_token();
        }
        Ok(program)
    }
    
    pub fn parse_statement(&mut self) -> Result<Statement, String> {
        match self.current_token {
            Token::Let => self.parse_let_statement(),
            _ => Err("Unexpected".to_string())
        }
    }
    
    pub fn parse_let_statement(&mut self) -> Result<Statement, String> {
        let token = self.current_token.clone();
        self.next_token();
        let identifier = if let Token::Ident(ref name) = self.current_token {
            Identifier(name.clone())
        } else {
            return Err(format!("Expected Ident, got {:?}", self.current_token));
        };
        self.next_token();
        if self.current_token != Token::Assign {
            return Err(format!("Expected Assign, got {:?}", self.current_token));
        }
        self.next_token();
        let expression = self.parse_expression()?;
        let statement = Statement::Let(LetStatement {
            token,
            name: identifier.clone(),
            value: expression,
        });
        Ok(statement)
    }
    
    pub fn parse_expression(&mut self) -> Result<Expression, String> {
        let expr = match self.current_token {
            Token::Int(ref value) => Ok(Expression::Lit(Literal(value.clone()))),
            Token::Ident(ref name) => Ok(Expression::Id(Identifier(name.clone()))),
            _ => Err(format!("Unexpected token: {:?}", self.current_token)),
        };
        self.next_token();
        expr
    }
}

#[cfg(test)]
mod tests {
    use crate::ast::{Expression, Identifier, LetStatement, Literal, Statement};
    use crate::lexer::Lexer;
    use crate::lexer::token::Token;
    use crate::parser::Parser;

    #[test]
    fn test_let_statements() -> Result<(), String> {
        let input = r#"
        let x = 5;
        let y = 10;
        let foobar = 838383;
        "#;
        
        let lexer = Lexer::new(input.to_string());
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program()?;
        assert_eq!(program.statements.len(), 3);
        let expected_statements: Vec<Statement> = vec![
            Statement::Let(LetStatement {
                token: Token::Let,
                name: Identifier("x".to_string()),
                value: Expression::Lit(Literal("5".to_string())),
            }),
            Statement::Let(LetStatement {
                token: Token::Let,
                name: Identifier("y".to_string()),
                value: Expression::Lit(Literal("10".to_string())),
            }),
            Statement::Let(LetStatement {
                token: Token::Let,
                name: Identifier("foobar".to_string()),
                value: Expression::Lit(Literal("838383".to_string())),
            }),
        ];
        
        for (i, statement) in program.statements.iter().enumerate() {
            assert_eq!(statement, &expected_statements[i]);
        }
        
        Ok(())
    }
}
    