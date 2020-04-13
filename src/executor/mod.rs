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
    symbol_table: HashMap<usize, DataTypes>,
    literal_table: HashMap<usize, String>,
    symbol_lookup_table: HashMap<String, usize>,
}

impl Executor {
    pub fn new(
        literal_table: HashMap<usize, String>,
        symbol_lookup_table: HashMap<String, usize>,
    ) -> Self {
        Executor {
            symbol_table: HashMap::new(),
            literal_table,
            symbol_lookup_table,
        }
    }

    pub fn get_symbol_name(&self, address: &usize) -> Option<String> {
        self.symbol_lookup_table.iter().find_map(|(key, val)| {
            if val == address {
                Some(key.to_string())
            } else {
                None
            }
        })
    }

    pub fn update_literal_table(&mut self, literal_table: HashMap<usize, String>) {
        self.literal_table = literal_table;
    }
    pub fn execute(
        &mut self,
        unit: &SourceUnit,
    ) -> Result<(), ((usize, usize), error::RunTimeErrors)> {
        for x in &unit.0 {
            let SourceUnitPart::Statement(stmt) = x;
            match stmt {
                Statement::Declaration((p, q), symbol) => {
                    if let Expression::Symbol((_a, _b), TokenType::Symbol(address)) = symbol {
                        if self
                            .symbol_table
                            .insert(*address, DataTypes::Unknown)
                            .is_some()
                        {
                            return Err((
                                (*p, *q),
                                RunTimeErrors::SymbolAlreadyDefined(
                                    self.get_symbol_name(address).unwrap(),
                                ),
                            ));
                        }
                    }
                }

                Statement::Conditional((_p, _q), expr, truebody, falsebody) => {
                    let truth = to_bool(self.eval_arithmetic_logic_expression(expr)?);
                    if truth {
                        self.execute(&truebody)?;
                    } else if let Some(body) = falsebody {
                        self.execute(&body)?;
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
                    if let Expression::Symbol((_a, _b), TokenType::Symbol(address)) = l {
                        if !self.symbol_table.contains_key(address) {
                            return Err((
                                (*p, *q),
                                RunTimeErrors::UndefinedSymbol(
                                    self.get_symbol_name(address).unwrap(),
                                ),
                            ));
                        }
                        self.symbol_table
                            .insert(*address, self.eval_arithmetic_logic_expression(&*r)?);
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

            Expression::Modulo((_a, _b), l, r) => Ok(self
                .eval_arithmetic_logic_expression(&**l)?
                % self.eval_arithmetic_logic_expression(&**r)?),

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

            Expression::Symbol((a, b), TokenType::Symbol(address)) => {
                //let identifier = (*identifier).to_string();
                if !self.symbol_table.contains_key(address) {
                    return Err((
                        (*a, *b),
                        RunTimeErrors::UndefinedSymbol(self.get_symbol_name(address).unwrap()),
                    ));
                }
                match self.symbol_table.get(address).unwrap() {
                    DataTypes::Integer(number) => Ok(DataTypes::Integer(*number)),
                    DataTypes::String(data) => Ok(DataTypes::String(data.to_string())),
                    _ => Err((
                        (*a, *b),
                        RunTimeErrors::UnInitialzedData(self.get_symbol_name(address).unwrap()),
                    )),
                }
            }
            Expression::Symbol((a, b), _) => Err(((*a, *b), RunTimeErrors::InvalidExpression)),

            Expression::StringLiteral((a, b), value) => {
                if let TokenType::Literal(address) = value {
                    Ok(DataTypes::String(
                        (&self.literal_table[address]).to_string(),
                    ))
                } else {
                    Err(((*a, *b), RunTimeErrors::InvalidExpression))
                }
            }

            Expression::InputNumber((a, b)) => {
                let mut input = String::new();

                if stdin().read_line(&mut input).is_err() {
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
