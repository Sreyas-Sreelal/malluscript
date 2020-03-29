use crate::ast::*;
///
/// TODO:Need to rewrite completely
///
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

    pub fn execute(&mut self, unit: SourceUnit) {
        for x in unit.0 {
            match x {
                SourceUnitPart::Expression(expr) => match expr {
                    _ => {}
                },
                SourceUnitPart::Statement(stmt) => match stmt {
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
                    _ => {}
                },
                _ => {}
            }
        }
    }
}
