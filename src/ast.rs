use crate::ast::show::Show;
use crate::lexer::token::Token;
pub mod show;

#[derive(Debug, PartialEq)]
pub enum Statement {
    Let(LetStatement),
    Return(ReturnStatement),
    Expression(ExpressionStatement),
}

impl Show for Statement {
    fn show(&self) -> String {
        match self {
            Statement::Let(let_statement) => let_statement.show(),
            Statement::Return(return_statement) => return_statement.show(),
            Statement::Expression(expression_statement) => expression_statement.show(),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Expression {
    Id(Identifier),
    Lit(Literal),
    Integer(IntegerLiteral),
    Prefix {
        operator: String,
        right: Box<Expression>,
    },
}

impl Show for Expression {
    fn show(&self) -> String {
        match self {
            Expression::Id(id) => id.0.clone(),
            Expression::Lit(lit) => lit.0.clone(),
            Expression::Integer(int) => int.0.to_string(),
            Expression::Prefix { operator, right } => format!("({}{})", operator, right.show()),
        }
    }
}

pub struct Program {
    pub statements: Vec<Statement>,
}

impl Program {
    pub fn new() -> Self {
        Self {
            statements: Vec::new(),
        }
    }
}

impl Show for Program {
    fn show(&self) -> String {
        let mut program = String::new();
        for statement in &self.statements {
            program.push_str(&statement.show());
        }
        program
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Identifier(pub String);

#[derive(Debug, PartialEq)]
pub struct Literal(pub String);

#[derive(Debug, PartialEq)]
pub struct IntegerLiteral(pub i64);

pub struct PrefixExpression {
    pub operator: String,
    pub right: Box<Expression>,
}

#[derive(Debug, PartialEq)]
pub struct LetStatement {
    pub token: Token,
    pub name: Identifier,
    pub value: Expression,
}

impl Show for LetStatement {
    fn show(&self) -> String {
        format!("{} {} = {};", self.token.literal, self.name.0, self.value.show())
    }
}

#[derive(Debug, PartialEq)]
pub struct ReturnStatement {
    pub token: Token,
    pub return_value: Expression,
}

impl Show for ReturnStatement {
    fn show(&self) -> String {
        format!("{} {};", self.token.literal, self.return_value.show())
    }
}

#[derive(Debug, PartialEq)]
pub struct ExpressionStatement {
    pub token: Token,
    pub expression: Expression,
}

impl Show for ExpressionStatement {
    fn show(&self) -> String {
        format!("{}", self.expression.show())
    }
}

#[cfg(test)]
mod tests {
    use crate::lexer::token::TokenType::{Let, Return};
    use super::*;

    #[test]
    fn test_show_program() {
        let program = Program {
            statements: vec![
                Statement::Let(LetStatement {
                    token: Token { token_type: Let, literal: "let".to_string() },
                    name: Identifier("myVar".to_string()),
                    value: Expression::Id(Identifier("anotherVar".to_string())),
                }),
                Statement::Return(ReturnStatement {
                    token: Token { token_type: Return, literal: "return".to_string() },
                    return_value: Expression::Lit(Literal("5".to_string())),
                }),
            ],
        };
        assert_eq!(program.show(), "let myVar = anotherVar;return 5;");
    }
}