use crate::lexer::TokenType;

#[derive(Debug)]
pub struct SourceUnit(pub Vec<SourceUnitPart>);

#[derive(Debug)]
pub enum SourceUnitPart {
    Statement(Statement),
}

#[derive(Debug)]
pub enum Statement {
    Conditional(Expression, SourceUnit, Option<SourceUnit>),
    Loop(Expression, SourceUnit),
    Declaration(TokenType),
    Assignment(TokenType, Expression),
    StringAlloc(TokenType, TokenType),
    WriteExpr(Expression),
    WriteString(TokenType),
}

#[derive(Debug)]
pub enum Expression {
    //    Empty,
    Add(Box<Expression>, Box<Expression>),
    Subtract(Box<Expression>, Box<Expression>),
    Multiply(Box<Expression>, Box<Expression>),
    Divide(Box<Expression>, Box<Expression>),
    UnaryMinus(Box<Expression>),
    Equals(Box<Expression>, Box<Expression>),
    GreaterThan(Box<Expression>, Box<Expression>),
    LessThan(Box<Expression>, Box<Expression>),
    NotEquals(Box<Expression>, Box<Expression>),
    Integer(TokenType),
    Symbol(TokenType),
}
