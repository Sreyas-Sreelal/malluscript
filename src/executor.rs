use crate::lexer::{Token, TokenType};
use meval::eval_str;
use std::collections::HashMap;
use std::mem::discriminant;

pub enum DataTypes {
    String(String),
    Integer(i32),
    Unknown,
}

pub struct Executor {
    symbol_table: HashMap<String, DataTypes>,
    lines: Vec<Vec<Token>>,
}

impl Executor {
    pub fn new(lines: Vec<Vec<Token>>) -> Self {
        Executor {
            symbol_table: HashMap::new(),
            lines,
        }
    }

    pub fn execute(&mut self) {
        for line in &self.lines {
            let mut x = 0;
            let length = line.len();
            while x < length {
                match &line[x].token {
                    TokenType::Write => {
                        x += 1;
                        if discriminant(&line[x].token)
                            == discriminant(&TokenType::Literal(String::new()))
                            || discriminant(&line[x].token)
                                == discriminant(&TokenType::Symbol(String::new()))
                        {
                            match &line[x].token {
                                TokenType::Literal(data) => {
                                    let data = &self.literal_eval(data.to_string());
                                    print!("{}", data);
                                }
                                TokenType::Symbol(variable) => {
                                    let data = &self.symbol_table[variable];
                                    match data {
                                        DataTypes::String(value) => {
                                            let value = &self.literal_eval(value.to_string());
                                            print!("{}", value);
                                        }
                                        DataTypes::Integer(value) => {
                                            print!("{}", value);
                                        }
                                        DataTypes::Unknown => {
                                            panic!(
                                                "Use of uninitialized variable {} at {}:{}",
                                                variable, line[x].line, line[x].offset
                                            );
                                        }
                                    }
                                }
                                _ => {
                                    panic!("Invalid value at {}:{}", line[x].line, line[x].offset);
                                }
                            }
                        } else {
                            panic!(
                                "Expected a literal or symbol at {}:{}",
                                line[x].line, line[x].offset
                            );
                        }
                    }
                    TokenType::Declaration => {
                        x += 1;
                        if discriminant(&line[x].token)
                            == discriminant(&TokenType::Symbol(String::new()))
                        {
                            if let TokenType::Symbol(data) = &line[x].token {
                                if !&self.symbol_table.contains_key(data) {
                                    &self
                                        .symbol_table
                                        .insert(data.to_string(), DataTypes::Unknown);
                                } else {
                                    panic!(
                                        "Duplicate Symbol {} at {}:{}",
                                        data, line[x].line, line[x].offset
                                    );
                                }
                            }
                        }
                    }
                    TokenType::Symbol(data) => {
                        //var = 1
                        if !&self.symbol_table.contains_key(data) {
                            panic!(
                                "Undefined Symbol  {} at {}:{}",
                                data, line[x].line, line[x].offset
                            );
                        }
                        x += 1;
                        if discriminant(&line[x].token) != discriminant(&TokenType::Assignment) {
                            panic!(
                                "Expected an assignment after symbol at {}:{}",
                                line[x].line, line[x].offset
                            );
                        }
                        x += 1;
                        match &line[x].token {
                            TokenType::Literal(value) => {
                                &self
                                    .symbol_table
                                    .insert(data.to_string(), DataTypes::String(value.to_string()));
                            }
                            TokenType::Number(_) | TokenType::Symbol(_) => {
                                let mut y = x;
                                let mut expr = String::new();
                                while y < length {
                                    let token = &line[y].token;
                                    let word = self.arithmetic(token);
                                    expr.push_str(&word);
                                    y += 1;
                                }
                                if let Ok(value) = eval_str(&expr) {
                                    self.symbol_table
                                        .insert(data.to_string(), DataTypes::Integer(value as i32));
                                    x = y;
                                } else {
                                    panic!(
                                        "Illegal expression at {}:{}",
                                        line[x].line, line[x].offset
                                    );
                                }
                            }
                            _ => {
                                panic!("Invalid value at {}:{}", line[x].line, line[x].offset);
                            }
                        }
                    }
                    TokenType::If => {
                        x += 1;
                    }
                    TokenType::Else => {
                        panic!(
                            "Else expression without If at {}:{}",
                            line[x].line, line[x].offset
                        );
                    }
                    _ => panic!(
                        "Unknown Token {:?} at {}:{}",
                        line[x].token, line[x].line, line[x].offset
                    ),
                }
                x += 1;
            }
        }
    }

    fn literal_eval(&self, data: String) -> String {
        data.replace("\\n", "\n")
    }

    fn arithmetic(&self, token: &TokenType) -> String {
        match token {
            TokenType::Plus => "+".to_string(),
            TokenType::Product => "*".to_string(),
            TokenType::Minus => "-".to_string(),
            TokenType::Divide => "/".to_string(),
            TokenType::Symbol(data) => match &self.symbol_table[data] {
                DataTypes::Integer(value) => value.to_string(),
                DataTypes::String(_) => {
                    panic!("Arithmetic operation with string is not yet supported");
                }
                DataTypes::Unknown => {
                    panic!("Use of uninitialized variable in the expression");
                }
            },
            TokenType::Number(data) => {
                let data = data.to_string();
                data
            }
            _ => {
                panic!("Illegal expression");
            }
        }
    }
}
