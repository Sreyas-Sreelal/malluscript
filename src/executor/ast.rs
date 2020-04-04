use crate::lexer::tokens::TokenType;

#[derive(Debug)]
pub struct SourceUnit<'input>(pub Vec<SourceUnitPart<'input>>);

#[derive(Debug)]
pub enum SourceUnitPart<'input> {
    Statement(Statement<'input>),
}

#[derive(Debug)]
pub enum Statement<'input> {
    Conditional(Expression<'input>, SourceUnit<'input>, Option<SourceUnit<'input>>),
    Loop(Expression<'input>, SourceUnit<'input>),
    Declaration(Expression<'input>),
    Assignment( Expression<'input>, Expression<'input>),
    Write(Expression<'input>),
}

#[derive(Debug)]
pub enum Expression<'input> {
    //    Empty,
    Add(Box<Expression<'input>>, Box<Expression<'input>>),
    Subtract(Box<Expression<'input>>, Box<Expression<'input>>),
    Multiply(Box<Expression<'input>>, Box<Expression<'input>>),
    Divide(Box<Expression<'input>>, Box<Expression<'input>>),
    UnaryMinus(Box<Expression<'input>>),
    Equals(Box<Expression<'input>>, Box<Expression<'input>>),
    GreaterThan(Box<Expression<'input>>, Box<Expression<'input>>),
    LessThan(Box<Expression<'input>>, Box<Expression<'input>>),
    NotEquals(Box<Expression<'input>>, Box<Expression<'input>>),
    Integer(TokenType<'input>),
    Symbol(&'input str),
    StringLiteral(&'input str),
    InputString,
    InputNumber
}
