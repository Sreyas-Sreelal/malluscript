use std::fs::File;
use std::io::{self, BufRead};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TokenType {
    Declaration,
    Write,
    InputString,
    InputNumber,
    LeftBrace,
    RightBrace,
    If,
    LOOP,
    Assignment,
    Plus,
    Minus,
    Product,
    Divide,
    Modulus,
    GreaterThan,
    LessThan,
    EqualTo,
    NotEqual,
    Literal(String),
    Number(i32),
    Symbol(String),
}

pub struct Token {
    pub line: usize,
    pub offset: usize,
    pub token: TokenType,
}

impl Token {
    pub fn new(token: TokenType, line: usize, offset: usize) -> Self {
        Token {
            line,
            offset,
            token,
        }
    }
}

pub fn tokenize(name: &str) -> Vec<Vec<Token>> {
    if let Ok(file) = File::open(name) {
        let lines = io::BufReader::new(file).lines();
        let mut compiled: Vec<Vec<Token>> = Vec::new();
        let mut lineno = 0;

        for line in lines {
            lineno += 1;
            let mut tokens: Vec<Token> = Vec::new();
            let mut offset: usize = 0;

            if let Ok(l) = line {
                let length = l.len();
                let l: Vec<char> = l.chars().collect();
                let mut x = 0;

                while x < length {
                    offset += 1;
                    let c = l[x];

                    match c {
                        ' ' => {
                            offset += 1;
                            x += 1;
                            continue;
                        }
                        '=' => tokens.push(Token::new(TokenType::Assignment, lineno, offset)),
                        '{' => tokens.push(Token::new(TokenType::LeftBrace, lineno, offset)),
                        '}' => tokens.push(Token::new(TokenType::RightBrace, lineno, offset)),
                        '+' => tokens.push(Token::new(TokenType::Plus, lineno, offset)),
                        '-' => tokens.push(Token::new(TokenType::Minus, lineno, offset)),
                        '*' => tokens.push(Token::new(TokenType::Product, lineno, offset)),
                        '/' => tokens.push(Token::new(TokenType::Divide, lineno, offset)),
                        '%' => tokens.push(Token::new(TokenType::Modulus, lineno, offset)),
                        '"' => {
                            x += 1;
                            let mut word = String::new();
                            while x < length && l[x] != '"' {
                                word.push(l[x]);
                                x += 1;
                            }
                            offset = x;

                            if l[x] != '"' {
                                panic!(
                                    "Invalid Literal Closing Quotation Not Found at {}:{}",
                                    lineno, offset
                                )
                            }
                            x += 1;
                            tokens.push(Token::new(TokenType::Literal(word), lineno, offset));
                        }
                        _ if c.is_alphabetic() => {
                            let mut word = String::new();
                            word.push(l[x]);
                            x += 1;
                            while x < length {
                                if l[x] == ' ' || !(l[x].is_ascii_alphanumeric() || l[x] == '_') {
                                    x -= 1;
                                    break;
                                }
                                word.push(l[x]);
                                x += 1;
                            }

                            match word.as_str() {
                                "pwoli_sadhanam" => {
                                    tokens.push(Token::new(TokenType::Declaration, lineno, offset))
                                }
                                "address_thada" => {
                                    tokens.push(Token::new(TokenType::InputString, lineno, offset))
                                }
                                "number_thada" => {
                                    tokens.push(Token::new(TokenType::InputNumber, lineno, offset))
                                }
                                "dhe_pidicho" => {
                                    tokens.push(Token::new(TokenType::Write, lineno, offset))
                                }
                                "seriyano_mwone" => {
                                    tokens.push(Token::new(TokenType::If, lineno, offset))
                                }
                                "repeat_adi" => {
                                    tokens.push(Token::new(TokenType::LOOP, lineno, offset))
                                }
                                "inekal_veluthane" => {
                                    tokens.push(Token::new(TokenType::GreaterThan, lineno, offset))
                                }
                                "um_same_alle" => {
                                    tokens.push(Token::new(TokenType::NotEqual, lineno, offset))
                                }
                                "inekal_cheruthane" => {
                                    tokens.push(Token::new(TokenType::LessThan, lineno, offset))
                                }
                                "um_same_aane" => {
                                    tokens.push(Token::new(TokenType::EqualTo, lineno, offset))
                                }
                                _ => {
                                    tokens.push(Token::new(TokenType::Symbol(word), lineno, offset))
                                }
                            }
                            offset = x;
                        }

                        _ if c.is_digit(10) => {
                            let mut word = String::new();
                            while x < length && l[x].is_digit(10) {
                                if l[x] == ' ' {
                                    break;
                                }
                                word.push(l[x]);
                                x += 1;
                            }
                            offset = x;
                            if let Ok(number) = word.parse() {
                                tokens.push(Token::new(TokenType::Number(number), lineno, offset));
                                continue;
                            } else {
                                panic!("Invalid literal {} at {} {}", word, lineno, offset);
                            }
                        }
                        _ => {
                            panic!("Oh no unknown token {} at {} {} ", c, lineno, offset);
                        }
                    }
                    x += 1;
                }
            }
            compiled.push(tokens);
        }
        compiled
    } else {
        panic!("File {} not found", name);
    }
}
