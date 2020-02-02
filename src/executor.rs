use crate::lexer::{Token, TokenType};
use meval::eval_str;
use std::collections::HashMap;
use std::mem::discriminant;
use std::process::exit;

#[derive(Debug, PartialEq)]
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
    stack: Vec<TokenType>,
    control_lines: Vec<usize>,
}

impl Executor {
    pub fn new(lines: Vec<Vec<Token>>) -> Self {
        Executor {
            symbol_table: HashMap::new(),
            lines,
            cur_row: 0,
            cur_col: 0,
            stack: Vec::new(),
            control_lines: Vec::new(),
        }
    }

    pub fn execute(&mut self) {
        let lines = self.lines.len();
        while self.cur_row < lines {
            self.cur_col = 0;
            let length = self.lines[self.cur_row].len();
            while self.cur_col < length {
                match &self.lines[self.cur_row][self.cur_col].token {
                    TokenType::Write => {
                        self.cur_col += 1;
                        if discriminant(&self.lines[self.cur_row][self.cur_col].token)
                            == discriminant(&TokenType::Literal(String::new()))
                            || discriminant(&self.lines[self.cur_row][self.cur_col].token)
                                == discriminant(&TokenType::Symbol(String::new()))
                        {
                            match &self.lines[self.cur_row][self.cur_col].token {
                                TokenType::Literal(data) => {
                                    let data = &self.literal_eval(data.to_string());
                                    print!("{}", data);
                                }

                                TokenType::Symbol(variable) => {
                                    if !&self.symbol_table.contains_key(variable) {
                                        self.throw_error(&format!(
                                            "Undefined Symbol  {}",
                                            variable
                                        ));
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
                        if discriminant(&self.lines[self.cur_row][self.cur_col].token)
                            == discriminant(&TokenType::Symbol(String::new()))
                        {
                            if let TokenType::Symbol(data) =
                                &self.lines[self.cur_row][self.cur_col].token
                            {
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
                        if discriminant(&self.lines[self.cur_row][self.cur_col].token)
                            != discriminant(&TokenType::Assignment)
                        {
                            self.throw_error("Expected an assignment after symbol");
                        }
                        self.cur_col += 1;
                        match &self.lines[self.cur_row][self.cur_col].token {
                            TokenType::Literal(value) => {
                                &self
                                    .symbol_table
                                    .insert(data.to_string(), DataTypes::String(value.to_string()));
                            }

                            TokenType::Number(_) | TokenType::Symbol(_) => {
                                let mut y = self.cur_col;
                                let mut expr = String::new();
                                while y < length {
                                    let token = &self.lines[self.cur_row][y].token;
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
                    TokenType::LOOP => {
                        let control = self.cur_row;
                        let success = self.parse_logical_expr();
                        if success {
                            self.stack
                                .push(self.lines[self.cur_row][self.cur_col].token.clone());
                            self.stack.push(TokenType::LOOP);
                            self.control_lines.push(control);
                        } else {
                            //skip loop
                            let mut level = 1;
                            while level != 0 {
                                self.cur_row += 1;
                                self.cur_col = 0;
                                while self.cur_col < self.lines[self.cur_row].len() {
                                    if discriminant(&TokenType::LeftBrace)
                                        == discriminant(
                                            &self.lines[self.cur_row][self.cur_col].token,
                                        )
                                    {
                                        level += 1;
                                    } else if discriminant(&TokenType::RightBrace)
                                        == discriminant(
                                            &self.lines[self.cur_row][self.cur_col].token,
                                        )
                                    {
                                        level -= 1;
                                    }
                                    self.cur_col += 1;
                                }
                            }
                            //self.cur_row +=1;
                            break;
                        }
                    }

                    TokenType::If => {
                        let success = self.parse_logical_expr();
                        if success {
                            self.stack
                                .push(self.lines[self.cur_row][self.cur_col].token.clone());
                        } else {
                            let mut level = 1;
                            while level != 0 {
                                self.cur_row += 1;
                                self.cur_col = 0;
                                while self.cur_col < self.lines[self.cur_row].len() {
                                    if discriminant(&TokenType::LeftBrace)
                                        == discriminant(
                                            &self.lines[self.cur_row][self.cur_col].token,
                                        )
                                    {
                                        level += 1;
                                    } else if discriminant(&TokenType::RightBrace)
                                        == discriminant(
                                            &self.lines[self.cur_row][self.cur_col].token,
                                        )
                                    {
                                        level -= 1;
                                    }
                                    self.cur_col += 1;
                                }
                            }
                            //self.cur_row+=1;
                            break;
                        }
                    }

                    TokenType::RightBrace => {
                        if self.cur_col != 0 {
                            self.throw_error("A block should end in new line with `}`")
                        }
                        if let Some(TokenType::LOOP) = self.stack.last() {
                            self.stack.pop();
                            self.stack.pop();
                            self.cur_row = self.control_lines.pop().unwrap() - 1;
                            break;
                        } else if let Some(TokenType::LeftBrace) = self.stack.last() {
                            self.stack.pop();
                        } else {
                            self.throw_error("Mismatched braces");
                        }
                    }
                    _ => self.throw_error(&format!(
                        "UnExpected Token {:?}",
                        self.lines[self.cur_row][self.cur_col].token
                    )),
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
        println!(
            "Oh Sed Noki chey monuse..Error at {}:{} => {}",
            &self.lines[self.cur_row][self.cur_col].line,
            &self.lines[self.cur_row][self.cur_col].offset,
            msg
        );
        exit(0);
    }

    fn parse_logical_expr(&mut self) -> bool {
        self.cur_col += 1;
        //TODO: Wrap them into a function
        let operand1 = match &self.lines[self.cur_row][self.cur_col].token {
            TokenType::Symbol(variable) => variable,
            _ => {
                self.throw_error("Expected a symbol");
            }
        };
        if !&self.symbol_table.contains_key(operand1) {
            self.throw_error(&format!("Undefined Symbol  {}", operand1));
        }

        self.cur_col += 1;
        let operand2 = match &self.lines[self.cur_row][self.cur_col].token {
            TokenType::Symbol(variable) => variable,
            _ => {
                self.throw_error("Expected a symbol");
            }
        };
        if !&self.symbol_table.contains_key(operand2) {
            self.throw_error(&format!("Undefined Symbol  {}", operand2));
        }

        let operand1 = &self.symbol_table[operand1];
        let operand2 = &self.symbol_table[operand2];
        if discriminant(operand1) != discriminant(operand2) {
            self.throw_error("Incompatible types");
        }

        self.cur_col += 1;
        let success = match &self.lines[self.cur_row][self.cur_col].token {
            TokenType::EqualTo => operand1 == operand2,
            TokenType::NotEqual => operand1 != operand2,
            TokenType::GreaterThan => {
                if let DataTypes::Integer(operand1) = operand1 {
                    if let DataTypes::Integer(operand2) = operand2 {
                        operand1 > operand2
                    } else {
                        self.throw_error("Only integers can be used for logical comparison")
                    }
                } else {
                    self.throw_error("Only integers can be used for logical comparison")
                }
            }
            TokenType::LessThan => {
                if let DataTypes::Integer(operand1) = operand1 {
                    if let DataTypes::Integer(operand2) = operand2 {
                        operand1 < operand2
                    } else {
                        self.throw_error("Only integers can be used for logical comparison")
                    }
                } else {
                    self.throw_error("Only integers can be used for logical comparison")
                }
            }
            e @ _ => {
                self.throw_error(&format!("Illegal operator {:?}", e));
            }
        };
        self.cur_col += 1;
        if discriminant(&self.lines[self.cur_row][self.cur_col].token)
            != discriminant(&TokenType::LeftBrace)
        {
            self.throw_error("Expected a `{`")
        }
        if self.cur_col + 1 != self.lines[self.cur_row].len() {
            self.throw_error("A block should start in new line after `{`")
        }
        success
    }
}
