use std::collections::HashMap;

use crate::ast::{Expression, ExpressionStatement, Identifier, IntegerLiteral, LetStatement, Program, ReturnStatement, Statement};
use crate::lexer::Lexer;
use crate::lexer::token::{Token, TokenType};
use crate::lexer::token::TokenType::{Bang, Dash, Ident, Illegal, Int};
use crate::parser::expression::{InfixParseFn, Precedence, PrefixParseFn};
use crate::parser::expression::Precedence::Lowest;
use crate::token;

pub mod expression;

pub struct Parser {
    lexer: Lexer,
    current_token: Token,
    peek_token: Token,
    prefix_parse_fns: HashMap<TokenType, PrefixParseFn>,
    infix_parse_fns: HashMap<TokenType, InfixParseFn>,
}

impl Parser {
    pub fn new(lexer: Lexer) -> Self {
        let mut parser = Self {
            lexer,
            current_token: token!(Illegal, ""),
            peek_token: token!(Illegal, ""),
            prefix_parse_fns: HashMap::new(),
            infix_parse_fns: HashMap::new(),
        };
        parser.prefix_parse_fns.insert(Ident, parse_identifier);
        parser.prefix_parse_fns.insert(Int, parse_integer_literal);
        parser.prefix_parse_fns.insert(Bang, parse_prefix_expression);
        parser.prefix_parse_fns.insert(Dash, parse_prefix_expression);
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
        while self.current_token.token_type != TokenType::Eof {
            let statement = self.parse_statement()?;
            program.statements.push(statement);
            self.next_token();
        }
        Ok(program)
    }
    
    pub fn parse_statement(&mut self) -> Result<Statement, String> {
        match self.current_token.token_type {
            TokenType::Let => self.parse_let_statement(),
            TokenType::Return => self.parse_return_statement(),
            _ => self.parse_expression_statement(),
        }
    }
    
    pub fn parse_let_statement(&mut self) -> Result<Statement, String> {
        let token = self.current_token.clone();
        self.next_token();
        let identifier = if let Token { token_type: TokenType::Ident, literal } = self.current_token.clone() {
            Identifier(literal.clone())
        } else {
            return Err(format!("Expected Ident, got {:?}", self.current_token));
        };
        self.next_token();
        if self.current_token.token_type != TokenType::Assign {
            return Err(format!("Expected Assign, got {:?}", self.current_token));
        }
        self.next_token();
        let expression = self.parse_expression(Lowest)?;
        let statement = Statement::Let(LetStatement {
            token,
            name: identifier.clone(),
            value: expression,
        });
        if self.peek_token.token_type == TokenType::Semicolon {
            self.next_token();
        } else {
            return Err(format!("Expected Semicolon, got {:?}", self.peek_token));
        }
        Ok(statement)
    }
    
    pub fn parse_return_statement(&mut self) -> Result<Statement, String> {
        let token = self.current_token.clone();
        self.next_token();
        let return_value = self.parse_expression(Lowest)?;
        let statement = Statement::Return(ReturnStatement {
            token,
            return_value,
        });
        if self.peek_token.token_type == TokenType::Semicolon {
            self.next_token();
        }
        Ok(statement)
    }
    
    pub fn parse_expression(&mut self, precedence: Precedence) -> Result<Expression, String> {
        let prefix = self.prefix_parse_fns.get(&self.current_token.token_type);
        if prefix.is_none() {
            return Err(format!("No prefix parse function for {:?}", self.current_token));
        }
        let left_expression = prefix.unwrap()(self)?;
        Ok(left_expression)
    }
    
    pub fn parse_expression_statement(&mut self) -> Result<Statement, String> {
        let expression = self.parse_expression(Lowest)?;
        let statement = Statement::Expression(ExpressionStatement {
            token: self.current_token.clone(),
            expression,
        });
        if self.peek_token.token_type == TokenType::Semicolon {
            self.next_token();
        }
        Ok(statement)
    }
}

pub fn parse_identifier(parser: &mut Parser) -> Result<Expression, String> {
    Ok(Expression::Id(Identifier(parser.current_token.literal.clone())))
}

pub fn parse_integer_literal(parser: &mut Parser) -> Result<Expression, String> {
    let val = parser.current_token.literal.parse::<i64>().unwrap();
    Ok(Expression::Integer(IntegerLiteral(val)))
}

pub fn parse_prefix_expression(parser: &mut Parser) -> Result<Expression, String> {
    let operator = parser.current_token.literal.clone();
    parser.next_token();
    let right = parser.parse_expression(Precedence::Prefix)?;
    Ok(Expression::Prefix {
        operator,
        right: Box::new(right),
    })
}

#[cfg(test)]
mod tests {
    use crate::ast::{Expression, ExpressionStatement, Identifier, IntegerLiteral, LetStatement, ReturnStatement, Statement};
    use crate::lexer::Lexer;
    use crate::lexer::token::Token;
    use crate::lexer::token::TokenType::{Ident, Int, Let, Return};
    use crate::parser::Parser;
    use crate::token;

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
                token: token!(Let, "let"),
                name: Identifier("x".to_string()),
                value: Expression::Integer(IntegerLiteral(5)),
            }),
            Statement::Let(LetStatement {
                token: token!(Let, "let"),
                name: Identifier("y".to_string()),
                value: Expression::Integer(IntegerLiteral(10)),
            }),
            Statement::Let(LetStatement {
                token: token!(Let, "let"),
                name: Identifier("foobar".to_string()),
                value: Expression::Integer(IntegerLiteral(838383)),
            }),
        ];
        
        for (i, statement) in program.statements.iter().enumerate() {
            assert_eq!(statement, &expected_statements[i]);
        }
        
        Ok(())
    }
    
    #[test]
    fn test_return_statements() -> Result<(), String> {
        let input = r#"
        return 5;
        return 10;
        return 993322;
        "#;
        
        let lexer = Lexer::new(input.to_string());
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program()?;
        assert_eq!(program.statements.len(), 3);
        let expected_statements: Vec<Statement> = vec![
            Statement::Return(ReturnStatement { token: token!(Return, "return"), return_value: Expression::Integer(IntegerLiteral(5)) }),
            Statement::Return(ReturnStatement { token: token!(Return, "return"), return_value: Expression::Integer(IntegerLiteral(10)) }),
            Statement::Return(ReturnStatement { token: token!(Return, "return"), return_value: Expression::Integer(IntegerLiteral(993322)) })
        ];
        
        for (i, statement) in program.statements.iter().enumerate() {
            assert_eq!(statement, &expected_statements[i]);
        }
        
        Ok(())
    }
    
    #[test]
    fn test_identifier_expression() -> Result<(), String> {
        let input = "foobar;";
        let lexer = Lexer::new(input.to_string());
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program()?;
        assert_eq!(program.statements.len(), 1);
        let expected_statement = Statement::Expression(ExpressionStatement { token: token!(Ident, "foobar"), expression: Expression::Id(Identifier("foobar".to_string())) });
        assert_eq!(program.statements[0], expected_statement);
        Ok(())
    }
    
    #[test]
    fn test_integer_expression() -> Result<(), String> {
        let input = "5;";
        let lexer = Lexer::new(input.to_string());
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program()?;
        assert_eq!(program.statements.len(), 1);
        let expected_statement = Statement::Expression(ExpressionStatement { token: token!(Int, "5"), expression: Expression::Integer(IntegerLiteral(5)) });
        assert_eq!(program.statements[0], expected_statement);
        Ok(())
    }
    
    #[test]
    fn test_prefix_expression() -> Result<(), String> {
        let input = "!5;-15;";
        let lexer = Lexer::new(input.to_string());
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program()?;
        assert_eq!(program.statements.len(), 2);
        let expected_statements: Vec<Statement> = vec![
            Statement::Expression(
                ExpressionStatement { 
                    token: token!(Int, "5"),
                    expression: Expression::Prefix { 
                        operator: "!".to_string(), 
                        right: Box::new(Expression::Integer(IntegerLiteral(5))) 
                    } 
                }
            ),
            Statement::Expression(
                ExpressionStatement { 
                    token: token!(Int, "15"), 
                    expression: Expression::Prefix { 
                        operator: "-".to_string(), 
                        right: Box::new(Expression::Integer(IntegerLiteral(15))) 
                    }
                }
            ),
        ];
        
        for (i, statement) in program.statements.iter().enumerate() {
            assert_eq!(statement, &expected_statements[i]);
        }
        
        Ok(())
    }
}
    