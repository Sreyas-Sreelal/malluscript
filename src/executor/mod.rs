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
    pub modules: HashMap<String, Executor>,
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
            modules: HashMap::new(),
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
                    Expression::ModuleAccess((_a, _b), module_id, expr) => {
                        let module_name = self.get_symbol_name(*module_id).unwrap();
                        let data = self.eval_arithmetic_logic_expression(r)?;
                        let mut evaluated_index = None;
                        if let Expression::ListSubScript((_x, _y), _, index) = &**expr {
                            evaluated_index =
                                Some(match self.eval_arithmetic_logic_expression(index) {
                                    Ok(DataTypes::Integer(idx)) => idx,
                                    _ => {
                                        return Err((
                                            (*p, *q),
                                            RunTimeErrors::IncompatibleOperation,
                                        ))
                                    }
                                });
                        }
                        let mut var_name = String::new();
                        if let Expression::Symbol((_x, _y), TokenType::Symbol(caller_address)) =
                            &**expr
                        {
                            var_name = self.get_symbol_name(*caller_address).unwrap();
                        }

                        if let Some(module) = self.modules.get_mut(&module_name) {
                            if let Expression::Symbol(
                                (_x, _y),
                                TokenType::Symbol(_caller_address),
                            ) = **expr
                            {
                                let target_address =
                                    if let Some(addr) = module.symbol_lookup_table.get(&var_name) {
                                        *addr
                                    } else {
                                        return Err((
                                            (*p, *q),
                                            RunTimeErrors::UndefinedSymbol(var_name),
                                        ));
                                    };

                                let mut frame_level = module.frame_level;
                                let mut data_address = target_address;

                                let left_type = module
                                    .symbol_table
                                    .get(&(module.frame_level, target_address));
                                if let Some(DataTypes::Ref((level, addr))) = left_type {
                                    frame_level = *level;
                                    data_address = *addr;
                                }
                                module
                                    .symbol_table
                                    .insert((frame_level, data_address), data);
                            } else if let Expression::ListSubScript((_x, _y), list_expr, _) =
                                &**expr
                            {
                                let mut indices = vec![evaluated_index.unwrap()];
                                let reference =
                                    module.evaluate_list_subscript(list_expr, &mut indices)?;
                                *reference = data;
                            } else {
                                return Err(((*p, *q), RunTimeErrors::InvalidAssignment));
                            }
                        } else {
                            return Err(((*p, *q), RunTimeErrors::UndefinedSymbol(module_name)));
                        }
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
                Statement::Import((p, q), path) => {
                    let mut module_path = String::new();
                    let mut module_name = String::new();
                    for (i, id) in path.iter().enumerate() {
                        let name = self.get_symbol_name(*id).unwrap();
                        if i > 0 {
                            module_path.push('/');
                        }
                        module_path.push_str(&name);
                        module_name = name;
                    }
                    module_path.push_str(".ms");

                    let source = match std::fs::read_to_string(&module_path) {
                        Ok(contents) => contents,
                        Err(e) => {
                            return Err((
                                (*p, *q),
                                RunTimeErrors::ModuleLoadError(format!(
                                    "Failed to read file {}: {}",
                                    module_path, e
                                )),
                            ))
                        }
                    };

                    let mut tokens = crate::lexer::Lexer::new(&source, HashMap::new(), 0);
                    let parsed = match crate::parser::parse(&source, &mut tokens) {
                        Ok(p) => p,
                        Err(e) => {
                            return Err((
                                (*p, *q),
                                RunTimeErrors::ModuleLoadError(format!(
                                    "Failed to parse module {}: {}",
                                    module_name, e
                                )),
                            ))
                        }
                    };

                    let mut exec = Executor::new(tokens.literal_table, tokens.symbol_lookup);
                    if let Err(e) = exec.execute(&parsed) {
                        return Err((
                            (*p, *q),
                            RunTimeErrors::ModuleLoadError(format!(
                                "Error executing module {}: {}",
                                module_name, e.1
                            )),
                        ));
                    }
                    self.modules.insert(module_name, exec);
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
                let mut is_module = false;
                let mut module_name = String::new();
                let func_name = match &**l {
                    Expression::Symbol((_a, _b), TokenType::Symbol(address)) => {
                        self.get_symbol_name(*address).unwrap()
                    }
                    Expression::ModuleAccess((_a, _b), module_id, expr) => {
                        module_name = self.get_symbol_name(*module_id).unwrap();
                        if let Expression::Symbol((_x, _y), TokenType::Symbol(address)) = &**expr {
                            is_module = true;
                            self.get_symbol_name(*address).unwrap()
                        } else {
                            return Err(((*p, *q), RunTimeErrors::InvalidExpression));
                        }
                    }
                    _ => return Err(((*p, *q), RunTimeErrors::InvalidExpression)),
                };

                let mut evaluated_args = Vec::new();
                for x in args.iter() {
                    let data = self.eval_arithmetic_logic_expression(x)?.clone();

                    let ref_info = if let (
                        DataTypes::List(_),
                        Expression::Symbol(_, TokenType::Symbol(x_addr)),
                    ) = (&data, x)
                    {
                        Some((self.frame_level, *x_addr))
                    } else {
                        None
                    };
                    evaluated_args.push((data, ref_info));
                }

                let function_data = if is_module {
                    let module = self.modules.get_mut(&module_name).ok_or((
                        (*p, *q),
                        RunTimeErrors::UndefinedSymbol(module_name.clone()),
                    ))?;
                    module.function_table.get(&func_name).cloned()
                } else {
                    self.function_table.get(&func_name).cloned()
                };

                if let Some(function) = function_data {
                    let parameters = &function.0;
                    if parameters.len() != evaluated_args.len() {
                        return Err(((*p, *q), RunTimeErrors::ArgumentCountMismatch));
                    }

                    let module = if is_module {
                        self.modules.get_mut(&module_name).unwrap() // Safe, checked earlier
                    } else {
                        self
                    };

                    for (i, y) in parameters.iter().enumerate() {
                        if let Expression::Symbol(_, TokenType::Symbol(y_addr)) = y {
                            let (data, ref_info) = evaluated_args[i].clone();
                            if !is_module && ref_info.is_some() {
                                let (level, addr) = ref_info.unwrap();
                                module.symbol_table.insert(
                                    (module.frame_level + 1, *y_addr),
                                    DataTypes::Ref((level, addr)),
                                );
                            } else {
                                module
                                    .symbol_table
                                    .insert((module.frame_level + 1, *y_addr), data);
                            }
                        } else {
                            return Err(((*p, *q), RunTimeErrors::InvalidFunctionDeclaration));
                        }
                    }

                    module.frame_level += 1;
                    module.return_storage = DataTypes::Unknown;
                    module.execute(&function.1)?;

                    let scope = module.frame_level;
                    module
                        .symbol_table
                        .retain(|(frame_level, _), _| *frame_level != scope);
                    module.frame_level -= 1;
                    module.subroutine_exit_flag = false;

                    Ok(module.return_storage.clone())
                } else {
                    return Err(((*p, *q), RunTimeErrors::UndefinedSymbol(func_name)));
                }
            }

            Expression::ModuleAccess((p, q), module_id, expr) => {
                let module_name = self.get_symbol_name(*module_id).unwrap();
                let mut var_name = String::new();
                if let Expression::Symbol((_a, _b), TokenType::Symbol(caller_address)) = &**expr {
                    var_name = self.get_symbol_name(*caller_address).unwrap();
                }

                if let Some(module) = self.modules.get_mut(&module_name) {
                    if let Expression::Symbol((a, b), TokenType::Symbol(_caller_address)) = **expr {
                        let target_address =
                            if let Some(addr) = module.symbol_lookup_table.get(&var_name) {
                                *addr
                            } else {
                                return Err(((*p, *q), RunTimeErrors::UndefinedSymbol(var_name)));
                            };
                        match module.eval_arithmetic_logic_expression(&Expression::Symbol(
                            (a, b),
                            TokenType::Symbol(target_address),
                        )) {
                            Ok(data) => Ok(data),
                            Err(e) => Err(((*p, *q), e.1)),
                        }
                    } else if let Expression::ListSubScript((_a, _b), _list_expr, _index) = &**expr
                    {
                        // Handled correctly in evaluate_list_subscript now
                        match module.eval_arithmetic_logic_expression(expr) {
                            Ok(data) => Ok(data),
                            Err(e) => Err(((*p, *q), e.1)),
                        }
                    } else {
                        Err(((*p, *q), RunTimeErrors::InvalidExpression))
                    }
                } else {
                    Err(((*p, *q), RunTimeErrors::UndefinedSymbol(module_name)))
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
        } else if let Expression::ModuleAccess((a, b), module_id, inner_expr) = expr {
            let module_name = self.get_symbol_name(*module_id).unwrap();
            let mut var_name = String::new();
            if let Expression::Symbol((_x, _y), TokenType::Symbol(caller_address)) = &**inner_expr {
                var_name = self.get_symbol_name(*caller_address).unwrap();
            }

            if let Some(module) = self.modules.get_mut(&module_name) {
                if let Expression::Symbol((_x, _y), TokenType::Symbol(_caller_address)) =
                    **inner_expr
                {
                    let target_address =
                        if let Some(addr) = module.symbol_lookup_table.get(&var_name) {
                            *addr
                        } else {
                            return Err(((*a, *b), RunTimeErrors::UndefinedSymbol(var_name)));
                        };
                    return module.evaluate_list_subscript(
                        &Expression::Symbol((_x, _y), TokenType::Symbol(target_address)),
                        vec,
                    );
                } else {
                    return Err(((*a, *b), RunTimeErrors::InvalidExpression));
                }
            } else {
                return Err(((*a, *b), RunTimeErrors::UndefinedSymbol(module_name)));
            }
        }
        Err(((0, 0), RunTimeErrors::InvalidExpression))
    }
}
