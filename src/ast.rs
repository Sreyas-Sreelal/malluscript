use crate::lexer::TokenType;

#[derive(Debug)]
pub struct SourceUnit(pub Vec<SourceUnitPart>);

#[derive(Debug)]
pub enum SourceUnitPart {
    Expression(Expression),
    Statement(Statement),
}

#[derive(Debug)]
pub enum Statement {
    Conditional(Expression, Box<Statement>, Option<Box<Statement>>),
    Loop(Expression, Box<Statement>),
    Declaration(TokenType),
    Allocation(Expression),
}

#[derive(Debug)]
pub enum Expression {
    Empty,
    Assignment(TokenType, Box<Expression>),
    StringAlloc(TokenType, TokenType),
    Add(Box<Expression>, Box<Expression>),
    Subtract(Box<Expression>, Box<Expression>),
    Multiply(Box<Expression>, Box<Expression>),
    Divide(Box<Expression>, Box<Expression>),
    Integer(TokenType),
    Symbol(TokenType),
}
