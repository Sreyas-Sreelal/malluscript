pub mod ast;
mod datatype;
mod error;

#[cfg(test)]
mod test;

use ast::*;
use error::RunTimeErrors;
use std::collections::HashMap;
use std::io::{stdin, stdout, Write};

use crate::executor::datatype::{to_bool, DataTypes};
use crate::lexer::tokens::TokenType;

pub struct Executor {
    symbol_table: HashMap<String, DataTypes>,
}

impl Executor {
    pub fn new() -> Self {
        Executor {
            symbol_table: HashMap::new(),
        }
    }

    pub fn execute(
        &mut self,
        unit: &SourceUnit,
    ) -> Result<(), ((usize, usize), error::RunTimeErrors)> {
        for x in &unit.0 {
            let SourceUnitPart::Statement(stmt) = x;
            match stmt {
                Statement::Declaration((p, q), symbol) => {
                    if let Expression::Symbol((_a, _b), name) = symbol {
                        if !self
                            .symbol_table
                            .insert(name.to_string(), DataTypes::Unknown)
                            .is_none()
                        {
                            return Err((
                                (*p, *q),
                                RunTimeErrors::SymbolAlreadyDefined(name.to_string()),
                            ));
                        }
                    }
                }

                Statement::Conditional((_p, _q), expr, truebody, falsebody) => {
                    let truth = to_bool(self.eval_arithmetic_logic_expression(expr)?);
                    if truth {
                        self.execute(&truebody)?;
                    } else {
                        if let Some(body) = falsebody {
                            self.execute(&body)?;
                        }
                    }
                }

                Statement::Loop((_p, _q), expr, body) => {
                    let mut truth = to_bool(self.eval_arithmetic_logic_expression(expr)?);
                    while truth {
                        self.execute(&body)?;
                        truth = to_bool(self.eval_arithmetic_logic_expression(expr)?);
                    }
                }

                Statement::Write((_p, _q), expr) => {
                    print!("{}", self.eval_arithmetic_logic_expression(expr)?);
                    let _ = stdout().flush();
                }

                Statement::Assignment((p, q), l, r) => {
                    if let Expression::Symbol((_a, _b), identifier) = l {
                        if !self.symbol_table.contains_key(&identifier.to_string()) {
                            return Err((
                                (*p, *q),
                                RunTimeErrors::UndefinedSymbol(identifier.to_string()),
                            ));
                        }
                        self.symbol_table.insert(
                            identifier.to_string(),
                            self.eval_arithmetic_logic_expression(&*r)?,
                        );
                    } else {
                        return Err(((*p, *q), RunTimeErrors::InvalidAssignment));
                    }
                }
            }
        }
        Ok(())
    }

    fn eval_arithmetic_logic_expression(
        &self,
        expr: &Expression,
    ) -> Result<DataTypes, ((usize, usize), RunTimeErrors)> {
        match expr {
            Expression::Add((_a, _b), l, r) => Ok(self.eval_arithmetic_logic_expression(&**l)?
                + self.eval_arithmetic_logic_expression(&**r)?),

            Expression::Multiply((_a, _b), l, r) => Ok(self
                .eval_arithmetic_logic_expression(&**l)?
                * self.eval_arithmetic_logic_expression(&**r)?),

            Expression::Subtract((_a, _b), l, r) => Ok(self
                .eval_arithmetic_logic_expression(&**l)?
                - self.eval_arithmetic_logic_expression(&**r)?),

            Expression::Divide((_a, _b), l, r) => Ok(self
                .eval_arithmetic_logic_expression(&**l)?
                / self.eval_arithmetic_logic_expression(&**r)?),

            Expression::UnaryMinus((_a, _b), r) => {
                Ok(DataTypes::Integer(-1) * self.eval_arithmetic_logic_expression(&**r)?)
            }

            Expression::Equals((_a, _b), l, r) => Ok(DataTypes::Bool(
                self.eval_arithmetic_logic_expression(&**l)?
                    == self.eval_arithmetic_logic_expression(&**r)?,
            )),

            Expression::NotEquals((_a, _b), l, r) => Ok(DataTypes::Bool(
                self.eval_arithmetic_logic_expression(&**l)?
                    != self.eval_arithmetic_logic_expression(&**r)?,
            )),

            Expression::GreaterThan((_a, _b), l, r) => Ok(DataTypes::Bool(
                self.eval_arithmetic_logic_expression(&**l)?
                    > self.eval_arithmetic_logic_expression(&**r)?,
            )),

            Expression::LessThan((_a, _b), l, r) => Ok(DataTypes::Bool(
                self.eval_arithmetic_logic_expression(&**l)?
                    < self.eval_arithmetic_logic_expression(&**r)?,
            )),

            Expression::Integer((a, b), l) => match l {
                TokenType::Number(number) => Ok(DataTypes::Integer(*number)),
                _ => Err(((*a, *b), RunTimeErrors::InvalidExpression)),
            },

            Expression::Symbol((a, b), identifier) => {
                let identifier = identifier.to_string();
                if !self.symbol_table.contains_key(&identifier) {
                    return Err(((*a, *b), RunTimeErrors::UndefinedSymbol(identifier)));
                }
                match self.symbol_table.get(&identifier).unwrap() {
                    DataTypes::Integer(number) => Ok(DataTypes::Integer(*number)),
                    DataTypes::String(data) => Ok(DataTypes::String(data.to_string())),
                    _ => Err(((*a, *b), RunTimeErrors::UnInitialzedData(identifier))),
                }
            }

            Expression::StringLiteral((_a, _b), value) => Ok(DataTypes::String(value.to_string())),

            Expression::InputNumber((a, b)) => {
                let mut input = String::new();

                if let Err(_) = stdin().read_line(&mut input) {
                    return Err(((*a, *b), RunTimeErrors::ErrorReadingStdin));
                }

                if let Ok(data) = input.trim().parse() {
                    Ok(DataTypes::Integer(data))
                } else {
                    Err(((*a, *b), RunTimeErrors::InvalidNumberInput))
                }
            }

            Expression::InputString((_a, _b)) => {
                let mut input = String::new();
                stdin().read_line(&mut input).expect("Unable to read input");
                Ok(DataTypes::String(input))
            }
        }
    }
}
