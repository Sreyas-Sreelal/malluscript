use crate::ast::*;
use crate::lexer::TokenType;
use std::collections::HashMap;
use std::io::{stdin, stdout, Write};
use std::mem::discriminant;
use std::process::exit;

#[derive(Debug, PartialEq, Clone)]
pub enum DataTypes {
    String(String),
    Integer(i64),
    Unknown,
}

pub struct Executor {
    symbol_table: HashMap<String, DataTypes>,
    cur_row: usize,
    cur_col: usize,
    stack: Vec<TokenType>,
}

impl Executor {
    pub fn new() -> Self {
        Executor {
            symbol_table: HashMap::new(),
            cur_row: 0,
            cur_col: 0,
            stack: Vec::new(),
        }
    }

    pub fn execute(&mut self, unit: &SourceUnit) {
        for x in &unit.0 {
            if let SourceUnitPart::Statement(stmt) = x {
                match stmt {
                    Statement::Declaration(symbol) => {
                        if let TokenType::Symbol(name) = symbol {
                            if !self
                                .symbol_table
                                .insert(name.clone(), DataTypes::Unknown)
                                .is_none()
                            {
                                panic!("Symbol {} already defiined", name)
                            }
                        }
                    }
                    Statement::Conditional(expr, truebody, falsebody) => {
                        let truth = (self.eval_arithmetic_logic_expression(expr).unwrap()) != 0;
                        if truth {
                            self.execute(&truebody);
                        } else {
                            if let Some(body) = falsebody {
                                self.execute(&body);
                            }
                        }
                    }
                    Statement::Loop(expr,body) => {
                        let mut truth = (self.eval_arithmetic_logic_expression(expr).unwrap()) != 0;
                        while truth {
                            self.execute(&body);
                            truth = (self.eval_arithmetic_logic_expression(expr).unwrap()) != 0;
                        }
                    }
                    Statement::WriteExpr(expr) => {
                        print!("{}", self.eval_arithmetic_logic_expression(expr).unwrap());
                    }
                    Statement::WriteString(token) => {
                        if let TokenType::Literal(string) = token {
                            println!("{}", string);
                        }
                    }
                    Statement::Allocation(expr) => match expr {
                        Expression::Assignment(l, r) => {
                            if let TokenType::Symbol(identifier) = l {
                                if !self.symbol_table.contains_key(identifier) {
                                    panic!("Undefined symbol {}", identifier);
                                }
                                //if let literal = Expression::
                                self.symbol_table.insert(
                                    identifier.to_string(),
                                    DataTypes::Integer(
                                        self.eval_arithmetic_logic_expression(&**r).unwrap(),
                                    ),
                                );
                            //println!("{:?}",self.symbol_table);
                            } else {
                                panic!("Invalid left assignment operator expected a symbol")
                            }
                        }
                        _ => {
                            //println!("somethinf elese {:?}", e);
                        }
                    },
                    _ => {}
                }
            }
        }
    }

    fn eval_arithmetic_logic_expression(&self, expr: &Expression) -> Result<i64, &'static str> {
        match expr {
            Expression::Add(l, r) => Ok(self.eval_arithmetic_logic_expression(&**l).unwrap()
                + self.eval_arithmetic_logic_expression(&**r).unwrap()),
            Expression::Multiply(l, r) => Ok(self.eval_arithmetic_logic_expression(&**l).unwrap()
                * self.eval_arithmetic_logic_expression(&**r).unwrap()),
            Expression::Subtract(l, r) => Ok(self.eval_arithmetic_logic_expression(&**l).unwrap()
                - self.eval_arithmetic_logic_expression(&**r).unwrap()),
            Expression::Divide(l, r) => Ok(self.eval_arithmetic_logic_expression(&**l).unwrap()
                / self.eval_arithmetic_logic_expression(&**r).unwrap()),
            Expression::Equals(l, r) => Ok((self.eval_arithmetic_logic_expression(&**l).unwrap()
                == self.eval_arithmetic_logic_expression(&**r).unwrap())
                as i64),
            Expression::NotEquals(l, r) => Ok((self.eval_arithmetic_logic_expression(&**l).unwrap()
                != self.eval_arithmetic_logic_expression(&**r).unwrap())
                as i64),
            Expression::GreaterThan(l, r) => {
                Ok((self.eval_arithmetic_logic_expression(&**l).unwrap()
                    > self.eval_arithmetic_logic_expression(&**r).unwrap()) as i64)
            }
            Expression::LessThan(l, r) => Ok((self.eval_arithmetic_logic_expression(&**l).unwrap()
                < self.eval_arithmetic_logic_expression(&**r).unwrap())
                as i64),
            Expression::Integer(l) => match l {
                TokenType::Number(number) => Ok(*number),
                _ => Err("Invalid constant"),
            },
            Expression::Symbol(token) => {
                if let TokenType::Symbol(identifier) = token {
                    if !self.symbol_table.contains_key(identifier) {
                        panic!("Undefined symbol {}", identifier);
                    }
                    if let DataTypes::Integer(number) = self.symbol_table.get(identifier).unwrap()
                    {
                        Ok(*number)
                    } else {
                        Err("Invalid constant in expression")
                    }
                } else {
                    Err("Invalid constant in expression")
                }
            }
            _ => Err("Invalid constant in expression"),
        }
    }
}
