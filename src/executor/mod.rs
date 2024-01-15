pub mod ast;
mod datatype;
mod error;

#[cfg(test)]
mod test;

use ast::*;
use error::RunTimeErrors;
use std::collections::{BTreeMap, HashMap};
use std::io::{stdin, stdout, Write};

use crate::executor::datatype::{to_bool, DataTypes};
use crate::lexer::tokens::TokenType;

pub type ScopeLevel = i64;

static GLOBAL_SCOPE: ScopeLevel = 0;

pub struct Executor {
    symbol_table: BTreeMap<(ScopeLevel, usize), DataTypes>,
    literal_table: HashMap<usize, String>,
    symbol_lookup_table: HashMap<String, usize>,
    function_table: HashMap<String, (Vec<Expression>, SourceUnit)>,
    frame_level: ScopeLevel,
    return_storage: DataTypes,
    subroutine_exit_flag: bool,
    pub output: Vec<String>,
}

impl Executor {
    pub fn new(
        literal_table: HashMap<usize, String>,
        symbol_lookup_table: HashMap<String, usize>,
    ) -> Self {
        Executor {
            symbol_table: BTreeMap::new(),
            literal_table,
            symbol_lookup_table,
            function_table: HashMap::new(),
            frame_level: GLOBAL_SCOPE,
            subroutine_exit_flag: false,
            return_storage: DataTypes::Integer(1),
            output: Vec::new(),
        }
    }

    pub fn get_symbol_name(&self, address: usize) -> Option<String> {
        self.symbol_lookup_table.iter().find_map(|(key, &val)| {
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
    pub fn update_lookup_table(&mut self, lookup_table: HashMap<String, usize>) {
        for x in lookup_table {
            self.symbol_lookup_table.insert(x.0, x.1);
        }
    }

    pub fn execute(
        &mut self,
        unit: &SourceUnit,
    ) -> Result<(), ((usize, usize), error::RunTimeErrors)> {
        //println!("{:?}",unit);

        for x in &unit.0 {
            if self.subroutine_exit_flag {
                continue; //return Ok(())
            }
            let SourceUnitPart::Statement(stmt) = x;
            match stmt {
                Statement::Conditional((_p, _q), expr, truebody, falsebody) => {
                    let truth = to_bool(self.eval_arithmetic_logic_expression(expr)?);
                    if truth {
                        self.execute(truebody)?;
                    } else if let Some(body) = falsebody {
                        self.execute(body)?;
                    }
                }

                Statement::Loop((_p, _q), expr, body) => {
                    let mut truth = to_bool(self.eval_arithmetic_logic_expression(expr)?);
                    while truth {
                        self.execute(body)?;
                        truth = to_bool(self.eval_arithmetic_logic_expression(expr)?);
                    }
                }

                Statement::Write(offset, expr) => {
                    let mut data = self.eval_arithmetic_logic_expression(expr)?;
                    if let DataTypes::Ref(address) = data {
                        data = self
                            .symbol_table
                            .get(&address)
                            .ok_or((*offset, RunTimeErrors::InvalidExpression))?
                            .clone();
                    }
                    print!("{}", data);
                    self.output.push(format!("{}", data));
                    let _ = stdout().flush();
                }

                Statement::Assignment((p, q), l, r) => match l {
                    Expression::Symbol((_a, _b), TokenType::Symbol(address)) => {
                        let mut frame_level = self.frame_level;
                        let mut data_address = *address;

                        let left_type = self.symbol_table.get(&(self.frame_level, *address));
                        if let Some(DataTypes::Ref((level, address))) = left_type {
                            frame_level = *level;
                            data_address = *address;
                        }

                        let data = self.eval_arithmetic_logic_expression(r)?;
                        self.symbol_table.insert((frame_level, data_address), data);
                    }
                    Expression::ListSubScript((_a, _b), expr, index) => {
                        let data = self.eval_arithmetic_logic_expression(r)?;
                        let index = match self.eval_arithmetic_logic_expression(index) {
                            Ok(DataTypes::Integer(idx)) => idx,
                            _ => return Err(((*p, *q), RunTimeErrors::IncompatibleOperation)),
                        };
                        let mut indices = Vec::new();
                        indices.push(index);
                        let reference = self.evaluate_list_subscript(expr, &mut indices)?;
                        *reference = data;
                    }
                    _ => {
                        return Err(((*p, *q), RunTimeErrors::InvalidAssignment));
                    }
                },
                Statement::EmptyExpression((_p, _q), expr) => {
                    self.eval_arithmetic_logic_expression(expr)?;
                }
                Statement::FunctionDeclaration((p, q), name, parameters, body) => {
                    if let Expression::Symbol((_a, _b), TokenType::Symbol(address)) = name {
                        let name = self.get_symbol_name(*address).unwrap();
                        if self.function_table.contains_key(&name) {
                            return Err(((*p, *q), RunTimeErrors::SymbolAlreadyDefined(name)));
                        }
                        self.function_table
                            .insert(name, (parameters.to_vec(), body.clone()));
                    } else {
                        return Err(((*p, *q), RunTimeErrors::InvalidFunctionDeclaration));
                    }
                }

                Statement::Return((_, _), expr) => {
                    self.return_storage = self.eval_arithmetic_logic_expression(expr)?;
                    self.subroutine_exit_flag = true;
                }
            }
        }
        Ok(())
    }

    fn eval_arithmetic_logic_expression(
        &mut self,
        expr: &Expression,
    ) -> Result<DataTypes, ((usize, usize), RunTimeErrors)> {
        match expr {
            Expression::Add(offset, l, r) => {
                let mut l = self.eval_arithmetic_logic_expression(l)?;
                let mut r = self.eval_arithmetic_logic_expression(r)?;
                if let DataTypes::Ref(address) = l {
                    l = self
                        .symbol_table
                        .get(&address)
                        .ok_or((*offset, RunTimeErrors::InvalidExpression))?
                        .clone();
                }
                if let DataTypes::Ref(address) = r {
                    r = self
                        .symbol_table
                        .get(&address)
                        .ok_or((*offset, RunTimeErrors::InvalidExpression))?
                        .clone();
                }
                Ok(l + r)
            }

            Expression::Multiply(offset, l, r) => {
                let mut l = self.eval_arithmetic_logic_expression(l)?;
                let mut r = self.eval_arithmetic_logic_expression(r)?;
                if let DataTypes::Ref(address) = l {
                    l = self
                        .symbol_table
                        .get(&address)
                        .ok_or((*offset, RunTimeErrors::InvalidExpression))?
                        .clone();
                }
                if let DataTypes::Ref(address) = r {
                    r = self
                        .symbol_table
                        .get(&address)
                        .ok_or((*offset, RunTimeErrors::InvalidExpression))?
                        .clone();
                }
                Ok(l * r)
            }

            Expression::Subtract(offset, l, r) => {
                let mut l = self.eval_arithmetic_logic_expression(l)?;
                let mut r = self.eval_arithmetic_logic_expression(r)?;
                if let DataTypes::Ref(address) = l {
                    l = self
                        .symbol_table
                        .get(&address)
                        .ok_or((*offset, RunTimeErrors::InvalidExpression))?
                        .clone();
                }
                if let DataTypes::Ref(address) = r {
                    r = self
                        .symbol_table
                        .get(&address)
                        .ok_or((*offset, RunTimeErrors::InvalidExpression))?
                        .clone();
                }
                Ok(l - r)
            }

            Expression::Divide(offset, l, r) => {
                let mut l = self.eval_arithmetic_logic_expression(l)?;
                let mut r = self.eval_arithmetic_logic_expression(r)?;
                if let DataTypes::Ref(address) = l {
                    l = self
                        .symbol_table
                        .get(&address)
                        .ok_or((*offset, RunTimeErrors::InvalidExpression))?
                        .clone();
                }
                if let DataTypes::Ref(address) = r {
                    r = self
                        .symbol_table
                        .get(&address)
                        .ok_or((*offset, RunTimeErrors::InvalidExpression))?
                        .clone();
                }
                Ok(l / r)
            }

            Expression::Modulo(offset, l, r) => {
                let mut l = self.eval_arithmetic_logic_expression(l)?;
                let mut r = self.eval_arithmetic_logic_expression(r)?;
                if let DataTypes::Ref(address) = l {
                    l = self
                        .symbol_table
                        .get(&address)
                        .ok_or((*offset, RunTimeErrors::InvalidExpression))?
                        .clone();
                }
                if let DataTypes::Ref(address) = r {
                    r = self
                        .symbol_table
                        .get(&address)
                        .ok_or((*offset, RunTimeErrors::InvalidExpression))?
                        .clone();
                }
                Ok(l % r)
            }

            Expression::UnaryMinus(offset, r) => {
                let mut r = self.eval_arithmetic_logic_expression(r)?;

                if let DataTypes::Ref(address) = r {
                    r = self
                        .symbol_table
                        .get(&address)
                        .ok_or((*offset, RunTimeErrors::InvalidExpression))?
                        .clone();
                }
                Ok(DataTypes::Integer(-1) * r)
            }

            Expression::Equals(offset, l, r) => {
                let mut l = self.eval_arithmetic_logic_expression(l)?;
                let mut r = self.eval_arithmetic_logic_expression(r)?;
                if let DataTypes::Ref(address) = l {
                    l = self
                        .symbol_table
                        .get(&address)
                        .ok_or((*offset, RunTimeErrors::InvalidExpression))?
                        .clone();
                }
                if let DataTypes::Ref(address) = r {
                    r = self
                        .symbol_table
                        .get(&address)
                        .ok_or((*offset, RunTimeErrors::InvalidExpression))?
                        .clone();
                }
                Ok(DataTypes::Bool(l == r))
            }

            Expression::NotEquals(offset, l, r) => {
                let mut l = self.eval_arithmetic_logic_expression(l)?;
                let mut r = self.eval_arithmetic_logic_expression(r)?;
                if let DataTypes::Ref(address) = l {
                    l = self
                        .symbol_table
                        .get(&address)
                        .ok_or((*offset, RunTimeErrors::InvalidExpression))?
                        .clone();
                }
                if let DataTypes::Ref(address) = r {
                    r = self
                        .symbol_table
                        .get(&address)
                        .ok_or((*offset, RunTimeErrors::InvalidExpression))?
                        .clone();
                }
                Ok(DataTypes::Bool(l != r))
            }

            Expression::GreaterThan(offset, l, r) => {
                let mut l = self.eval_arithmetic_logic_expression(l)?;
                let mut r = self.eval_arithmetic_logic_expression(r)?;
                if let DataTypes::Ref(address) = l {
                    l = self
                        .symbol_table
                        .get(&address)
                        .ok_or((*offset, RunTimeErrors::InvalidExpression))?
                        .clone();
                }
                if let DataTypes::Ref(address) = r {
                    r = self
                        .symbol_table
                        .get(&address)
                        .ok_or((*offset, RunTimeErrors::InvalidExpression))?
                        .clone();
                }
                Ok(DataTypes::Bool(l > r))
            }

            Expression::LessThan(offset, l, r) => {
                let mut l = self.eval_arithmetic_logic_expression(l)?;
                let mut r = self.eval_arithmetic_logic_expression(r)?;
                if let DataTypes::Ref(address) = l {
                    l = self
                        .symbol_table
                        .get(&address)
                        .ok_or((*offset, RunTimeErrors::InvalidExpression))?
                        .clone();
                }
                if let DataTypes::Ref(address) = r {
                    r = self
                        .symbol_table
                        .get(&address)
                        .ok_or((*offset, RunTimeErrors::InvalidExpression))?
                        .clone();
                }
                Ok(DataTypes::Bool(l < r))
            }

            Expression::Integer((a, b), l) => match l {
                TokenType::Integer(number) => Ok(DataTypes::Integer(*number)),
                _ => Err(((*a, *b), RunTimeErrors::InvalidExpression)),
            },
            Expression::Float((a, b), l) => match l {
                TokenType::Float(number) => Ok(DataTypes::Float(*number)),
                _ => Err(((*a, *b), RunTimeErrors::InvalidExpression)),
            },
            Expression::Symbol((a, b), TokenType::Symbol(address)) => {
                let mut level = self.frame_level;

                if !self.symbol_table.contains_key(&(level, *address)) {
                    level = 0;
                }

                match self.symbol_table.get(&(level, *address)) {
                    Some(DataTypes::Integer(number)) => Ok(DataTypes::Integer(*number)),
                    Some(DataTypes::Float(number)) => Ok(DataTypes::Float(*number)),
                    Some(DataTypes::String(data)) => Ok(DataTypes::String(data.to_string())),
                    Some(DataTypes::List(data)) => Ok(DataTypes::List(data.to_vec())),
                    Some(DataTypes::Ref((level, address))) => {
                        Ok(DataTypes::Ref((*level, *address)))
                    }
                    _ => Err((
                        (*a, *b),
                        RunTimeErrors::UndefinedSymbol(self.get_symbol_name(*address).unwrap()),
                    )),
                }
            }

            Expression::Symbol((a, b), _) => Err(((*a, *b), RunTimeErrors::InvalidExpression)),

            Expression::StringLiteral((a, b), value) => {
                if let TokenType::Literal(address) = value {
                    Ok(DataTypes::String(self.literal_table[address].to_string()))
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
                } else if let Ok(data) = input.trim().parse() {
                    Ok(DataTypes::Float(data))
                } else {
                    Err(((*a, *b), RunTimeErrors::InvalidNumberInput))
                }
            }

            Expression::InputString((_a, _b)) => {
                let mut input = String::new();
                stdin().read_line(&mut input).expect("Unable to read input");
                Ok(DataTypes::String(input))
            }

            Expression::FunctionCall((p, q), l, args) => {
                if let Expression::Symbol((_a, _b), TokenType::Symbol(address)) = **l {
                    let name = self.get_symbol_name(address).unwrap();
                    if let Some(function) = self.function_table.get(&name) {
                        // See: https://github.com/rust-lang/rust/issues/59159
                        let function = function.clone();

                        let parameters = &function.0;
                        if parameters.len() != args.len() {
                            return Err(((*p, *q), RunTimeErrors::ArgumentCountMismatch));
                        }
                        for (x, y) in args.iter().zip(parameters.iter()) {
                            if let Expression::Symbol(_, TokenType::Symbol(y)) = y {
                                // allocation
                                let data = self.eval_arithmetic_logic_expression(x)?.clone();
                                if let (
                                    DataTypes::List(_),
                                    Expression::Symbol(_, TokenType::Symbol(x)),
                                ) = (&data, x)
                                {
                                    // lists will be passed by reference by default
                                    self.symbol_table.insert(
                                        (self.frame_level + 1, *y),
                                        DataTypes::Ref((self.frame_level, *x)),
                                    );
                                } else {
                                    self.symbol_table.insert((self.frame_level + 1, *y), data);
                                }
                            } else {
                                return Err(((*p, *q), RunTimeErrors::InvalidFunctionDeclaration));
                            }
                        }

                        self.frame_level += 1;
                        self.return_storage = DataTypes::Unknown;
                        self.execute(&function.1)?;

                        let scope = self.frame_level;
                        self.symbol_table
                            .retain(|(frame_level, _), _| *frame_level != scope);
                        self.frame_level -= 1;
                    } else {
                        return Err(((*p, *q), RunTimeErrors::UndefinedSymbol(name)));
                    }
                    self.subroutine_exit_flag = false;
                    Ok(self.return_storage.clone())
                } else {
                    Err(((*p, *q), RunTimeErrors::InvalidExpression))
                }
            }

            Expression::ListExpression((_p, _q), items) => {
                let mut list = Vec::new();
                for item in items {
                    let data = self.eval_arithmetic_logic_expression(item)?;
                    list.push(data);
                }
                Ok(DataTypes::List(list))
            }

            Expression::ListSubScript((p, q), expr, index) => {
                let index = match self.eval_arithmetic_logic_expression(index) {
                    Ok(DataTypes::Integer(idx)) => idx,
                    _ => return Err(((*p, *q), RunTimeErrors::IncompatibleOperation)),
                };
                let mut indices = Vec::new();
                indices.push(index);
                let reference = self.evaluate_list_subscript(expr, &mut indices)?;
                Ok(reference.clone())
            }
        }
    }

    pub fn evaluate_list_subscript(
        &mut self,
        expr: &Expression,
        vec: &mut Vec<i64>,
    ) -> Result<&mut DataTypes, ((usize, usize), error::RunTimeErrors)> {
        if let Expression::ListSubScript((p, q), expr, index) = expr {
            let index = match self.eval_arithmetic_logic_expression(index) {
                Ok(DataTypes::Integer(idx)) => idx,
                _ => return Err(((*p, *q), RunTimeErrors::IncompatibleOperation)),
            };
            vec.push(index);
            return self.evaluate_list_subscript(expr, vec);
        } else if let Expression::Symbol((a, b), TokenType::Symbol(address)) = expr {
            let mut frame_level = self.frame_level;
            let mut data_address = address;
            let left_type = self.symbol_table.get(&(self.frame_level, *address));
            if let Some(DataTypes::Ref((level, address))) = left_type {
                frame_level = *level;
                data_address = address;
            }
            let mut data = self.symbol_table.get_mut(&(frame_level, *data_address));
            while let Some(index) = vec.pop() {
                if let Some(DataTypes::List(list)) = data {
                    if index < 0 || index > (list.len() - 1) as i64 {
                        return Err((
                            (*a, *b),
                            RunTimeErrors::IndexOutOfBounds(index, list.len() as i64),
                        ));
                    }
                    data = Some(&mut list[index as usize]);
                } else {
                    return Err(((*a, *b), RunTimeErrors::InvalidExpression));
                }
            }
            return Ok(data.unwrap());
        }
        Err(((0, 0), RunTimeErrors::InvalidExpression))
    }
}
