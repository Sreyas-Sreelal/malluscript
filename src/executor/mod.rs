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
use libloading::{Library, Symbol};
use crate::lexer::tokens::TokenType;

pub type ScopeLevel = i64;

static GLOBAL_SCOPE: ScopeLevel = 0;

pub enum FunctionDef {
    Script(Vec<Expression>, SourceUnit),
    Native(fn(&mut Executor, Vec<DataTypes>) -> Result<DataTypes, RunTimeErrors>),
}

pub struct Executor {
    symbol_table: BTreeMap<(ScopeLevel, usize), DataTypes>,
    literal_table: HashMap<usize, String>,
    symbol_lookup_table: HashMap<String, usize>,
    function_table: HashMap<String, FunctionDef>,
    native_libraries: std::collections::HashMap<usize, Library>,
    native_symbols: std::collections::HashMap<usize, Symbol<unsafe extern "C" fn(i64) -> i64>>,
    next_lib_handle: usize,
    next_sym_handle: usize,
    frame_level: ScopeLevel,
    return_storage: DataTypes,
    subroutine_exit_flag: bool,
}

impl Executor {
    pub fn new(
        literal_table: HashMap<usize, String>,
        symbol_lookup_table: HashMap<String, usize>,
    ) -> Self {
        let mut executor = Executor {
            symbol_table: BTreeMap::new(),
            literal_table,
            symbol_lookup_table,
            function_table: HashMap::new(),
            native_libraries: HashMap::new(),
            native_symbols: HashMap::new(),
            next_lib_handle: 1,
            next_sym_handle: 1,
            frame_level: GLOBAL_SCOPE,
            return_storage: DataTypes::Integer(1),
            subroutine_exit_flag: false,
        };
        executor.register_native_functions();
        executor
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
    
    fn register_native_functions(&mut self) {
        self.function_table.insert(
            "ffi_open".to_string(),
            FunctionDef::Native(Executor::native_ffi_open),
        );
        self.function_table.insert(
            "ffi_sym".to_string(),
            FunctionDef::Native(Executor::native_ffi_sym),
        );
        self.function_table.insert(
            "ffi_call".to_string(),
            FunctionDef::Native(Executor::native_ffi_call),
        );
    }

    fn native_ffi_open(&mut self, args: Vec<DataTypes>) -> Result<DataTypes, RunTimeErrors> {
        if args.len() != 1 {
            return Err(RunTimeErrors::ArgumentCountMismatch);
        }
        if let DataTypes::String(path) = &args[0] {
            match Library::new(path) {
                Ok(lib) => {
                    let handle = self.next_lib_handle;
                    self.native_libraries.insert(handle, lib);
                    self.next_lib_handle += 1;
                    Ok(DataTypes::Integer(handle as i64))
                }
                Err(err) => Err(RunTimeErrors::FFIOpenError(err.to_string())),
            }
        } else {
            Err(RunTimeErrors::IncompatibleOperation)
        }
    }

    fn native_ffi_sym(&mut self, args: Vec<DataTypes>) -> Result<DataTypes, RunTimeErrors> {
        if args.len() != 2 {
            return Err(RunTimeErrors::ArgumentCountMismatch);
        }
        if let (DataTypes::Integer(lib_handle), DataTypes::String(sym)) = (&args[0], &args[1]) {
            if let Some(lib) = self.native_libraries.get(&(*lib_handle as usize)) {
                unsafe {
                    match lib.get::<unsafe extern "C" fn(i64) -> i64>(sym.as_bytes()) {
                        Ok(symbol) => {
                            let handle = self.next_sym_handle;
                            self.native_symbols.insert(handle, symbol);
                            self.next_sym_handle += 1;
                            Ok(DataTypes::Integer(handle as i64))
                        }
                        Err(err) => Err(RunTimeErrors::FFISymError(err.to_string())),
                    }
                }
            } else {
                Err(RunTimeErrors::FFIOpenError(format!(
                    "Invalid library handle {}", lib_handle
                )))
            }
        } else {
            Err(RunTimeErrors::IncompatibleOperation)
        }
    }

    fn native_ffi_call(&mut self, args: Vec<DataTypes>) -> Result<DataTypes, RunTimeErrors> {
        if args.len() != 2 {
            return Err(RunTimeErrors::ArgumentCountMismatch);
        }
        if let (DataTypes::Integer(sym_handle), DataTypes::Integer(val)) = (&args[0], &args[1]) {
            if let Some(symbol) = self.native_symbols.get(&(*sym_handle as usize)) {
                let result = unsafe { symbol(*val) };
                Ok(DataTypes::Integer(result))
            } else {
                Err(RunTimeErrors::FFISymError(format!(
                    "Invalid symbol handle {}", sym_handle
                )))
            }
        } else {
            Err(RunTimeErrors::IncompatibleOperation)
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

            Expression::FunctionCall((p, q), func_expr, args) => {
                // Evaluate function name
                if let Expression::Symbol((_a, _b), TokenType::Symbol(address)) = **func_expr {
                    let name = self.get_symbol_name(address).unwrap();
                    if let Some(def) = self.function_table.get(&name) {
                        match def {
                            FunctionDef::Script(params, body) => {
                                if params.len() != args.len() {
                                    return Err(((*p, *q), RunTimeErrors::ArgumentCountMismatch));
                                }
                                // Setup parameters
                                for (arg_expr, param_expr) in args.iter().zip(params.iter()) {
                                    if let Expression::Symbol(_, TokenType::Symbol(param_addr)) = param_expr {
                                        let data = self.eval_arithmetic_logic_expression(arg_expr)?.clone();
                                        if let (DataTypes::List(_), Expression::Symbol(_, TokenType::Symbol(arg_addr))) =
                                            (&data, arg_expr)
                                        {
                                            self.symbol_table.insert(
                                                (self.frame_level + 1, *param_addr),
                                                DataTypes::Ref((self.frame_level, *arg_addr)),
                                            );
                                        } else {
                                            self.symbol_table.insert((self.frame_level + 1, *param_addr), data);
                                        }
                                    } else {
                                        return Err(((*p, *q), RunTimeErrors::InvalidFunctionDeclaration));
                                    }
                                }
                                // Execute script function
                                self.frame_level += 1;
                                self.return_storage = DataTypes::Unknown;
                                self.execute(body)?;
                                let scope = self.frame_level;
                                self.symbol_table.retain(|(lvl, _), _| *lvl != scope);
                                self.frame_level -= 1;
                                self.subroutine_exit_flag = false;
                                Ok(self.return_storage.clone())
                            }
                            FunctionDef::Native(native_fn) => {
                                // Evaluate arguments first
                                let mut native_args = Vec::new();
                                for arg_expr in args {
                                    native_args.push(self.eval_arithmetic_logic_expression(arg_expr)?);
                                }
                                // Call native function
                                native_fn(self, native_args)
                            }
                        }
                    } else {
                        Err(((*p, *q), RunTimeErrors::UndefinedSymbol(name)))
                    }
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
