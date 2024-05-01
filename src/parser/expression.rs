use crate::ast::Expression;
use crate::parser::Parser;

pub type PrefixParseFn = fn(&mut Parser) -> Result<Expression, String>;
pub type InfixParseFn = fn(&mut Parser, Expression) -> Result<Expression, String>;

#[derive(Debug, PartialEq, Clone, PartialOrd)]
pub enum Precedence {
    Lowest,
    Equals,
    LessGreater,
    Sum,
    Product,
    Prefix,
    Call,
}