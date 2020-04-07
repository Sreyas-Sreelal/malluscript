use crate::lexer::tokens::TokenType;

#[derive(Debug, Clone, PartialEq)]
pub struct SourceUnit<'input>(pub Vec<SourceUnitPart<'input>>);

#[derive(Debug, Clone, PartialEq)]
pub enum SourceUnitPart<'input> {
    Statement(Statement<'input>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Statement<'input> {
    Conditional(
        (usize, usize),
        Expression<'input>,
        SourceUnit<'input>,
        Option<SourceUnit<'input>>,
    ),
    Loop((usize, usize), Expression<'input>, SourceUnit<'input>),
    Declaration((usize, usize), Expression<'input>),
    Assignment((usize, usize), Expression<'input>, Expression<'input>),
    Write((usize, usize), Expression<'input>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expression<'input> {
    //    Empty,
    Add(
        (usize, usize),
        Box<Expression<'input>>,
        Box<Expression<'input>>,
    ),
    Subtract(
        (usize, usize),
        Box<Expression<'input>>,
        Box<Expression<'input>>,
    ),
    Multiply(
        (usize, usize),
        Box<Expression<'input>>,
        Box<Expression<'input>>,
    ),
    Divide(
        (usize, usize),
        Box<Expression<'input>>,
        Box<Expression<'input>>,
    ),
    UnaryMinus((usize, usize), Box<Expression<'input>>),
    Equals(
        (usize, usize),
        Box<Expression<'input>>,
        Box<Expression<'input>>,
    ),
    GreaterThan(
        (usize, usize),
        Box<Expression<'input>>,
        Box<Expression<'input>>,
    ),
    LessThan(
        (usize, usize),
        Box<Expression<'input>>,
        Box<Expression<'input>>,
    ),
    NotEquals(
        (usize, usize),
        Box<Expression<'input>>,
        Box<Expression<'input>>,
    ),
    Integer((usize, usize), TokenType<'input>),
    Symbol((usize, usize), &'input str),
    StringLiteral((usize, usize), &'input str),
    InputString((usize, usize)),
    InputNumber((usize, usize)),
}
