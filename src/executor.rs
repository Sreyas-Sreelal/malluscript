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
    cur_row: usize,
    cur_col: usize,
    stack:Vec<Token>
}

impl Executor {
    pub fn new(lines: Vec<Vec<Token>>) -> Self {
        Executor {
            symbol_table: HashMap::new(),
            lines,
            cur_row: 0,
            cur_col: 0,
            stack:Vec::new(),
        }
    }

    pub fn execute(&mut self) {
        for line in &self.lines {
            self.cur_col = 0;
            let length = line.len();
            while self.cur_col < length {
                match &line[self.cur_col].token {
                    TokenType::Write => {
                        self.cur_col += 1;
                        if discriminant(&line[self.cur_col].token)
                            == discriminant(&TokenType::Literal(String::new()))
                            || discriminant(&line[self.cur_col].token)
                                == discriminant(&TokenType::Symbol(String::new()))
                        {
                            match &line[self.cur_col].token {
                                TokenType::Literal(data) => {
                                    let data = &self.literal_eval(data.to_string());
                                    print!("{}", data);
                                }
                                TokenType::Symbol(variable) => {
                                    if !&self.symbol_table.contains_key(variable) {
                                        self.throw_error(&format!("Undefined Symbol  {}", variable));
                                    }
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
                                            self.throw_error(&format!(
                                                "Use of uninitialized variable {}",
                                                variable
                                            ));
                                        }
                                    }
                                }
                                _ => {
                                    self.throw_error("Invalid value");
                                }
                            }
                        } else {
                            self.throw_error("Expected a literal or symbol");
                        }
                    }
                    TokenType::Declaration => {
                        self.cur_col += 1;
                        if discriminant(&line[self.cur_col].token)
                            == discriminant(&TokenType::Symbol(String::new()))
                        {
                            if let TokenType::Symbol(data) = &line[self.cur_col].token {
                                if !&self.symbol_table.contains_key(data) {
                                    &self
                                        .symbol_table
                                        .insert(data.to_string(), DataTypes::Unknown);
                                } else {
                                    self.throw_error(&format!("Duplicate Symbol {}", data));
                                }
                            }
                        }
                    }
                    TokenType::Symbol(data) => {
                        //var = 1
                        if !&self.symbol_table.contains_key(data) {
                            self.throw_error(&format!("Undefined Symbol  {}", data));
                        }
                        self.cur_col += 1;
                        if discriminant(&line[self.cur_col].token)
                            != discriminant(&TokenType::Assignment)
                        {
                            self.throw_error("Expected an assignment after symbol");
                        }
                        self.cur_col += 1;
                        match &line[self.cur_col].token {
                            TokenType::Literal(value) => {
                                &self
                                    .symbol_table
                                    .insert(data.to_string(), DataTypes::String(value.to_string()));
                            }
                            TokenType::Number(_) | TokenType::Symbol(_) => {
                                let mut y = self.cur_col;
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
                                    self.cur_col = y;
                                } else {
                                    self.throw_error("Illegal expression");
                                }
                            }
                            _ => {
                                self.throw_error("Invalid value");
                            }
                        }
                    }
                    //TODO
                    TokenType::If => {
                        self.cur_col += 1;
                    }
                    TokenType::Else => {
                        self.throw_error("Else expression without If");
                    }
                    _ => self.throw_error(&format!("Unknown Token {:?}", line[self.cur_col].token)),
                }
                self.cur_col += 1;
            }
            self.cur_row += 1;
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
                    self.throw_error("Arithmetic operation with string is not yet supported");
                }
                DataTypes::Unknown => {
                    self.throw_error("Use of uninitialized variable in the expression");
                }
            },
            TokenType::Number(data) => {
                let data = data.to_string();
                data
            }
            _ => {
                self.throw_error("Illegal expression");
            }
        }
    }

    fn throw_error(&self, msg: &str) -> ! {
        panic!(
            "[{}:{}] => {}",
            &self.lines[self.cur_row][self.cur_col].line,
            &self.lines[self.cur_row][self.cur_col].offset,
            msg
        );
    }
}
