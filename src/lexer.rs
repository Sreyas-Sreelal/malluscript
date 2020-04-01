///
/// Guhiki Lexer
///
use std::str::CharIndices;

#[derive(Clone)]
pub struct Lexer<'input> {
    chars: CharIndices<'input>,
}

impl<'input> Lexer<'input> {
    pub fn new(input: &'input str) -> Self {
        Lexer {
            chars: input.char_indices(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TokenType {
    Declaration,
    Write,
    InputString,
    InputNumber,
    LeftBrace,
    RightBrace,
    If,
    Else,
    Loop,
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
    SemiColon,
    OpenParantheses,
    CloseParantheses,
    Literal(String),
    Number(i64),
    Symbol(String),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum LexicalError {
    //TODO:add custom lexical errors instead of panicking
}

pub type Spanned<TokenType, Loc, Error> = Result<(Loc, TokenType, Loc), Error>;

impl<'input> Iterator for Lexer<'input> {
    type Item = Spanned<TokenType, usize, LexicalError>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.chars.next() {
                Some((_, ' ')) | Some((_, '\n')) => {
                    continue;
                }
                Some((i, '=')) => return Some(Ok((i, TokenType::Assignment, i))),
                Some((i, '{')) => return Some(Ok((i, TokenType::LeftBrace, i))),
                Some((i, '}')) => return Some(Ok((i, TokenType::RightBrace, i))),
                Some((i, '+')) => return Some(Ok((i, TokenType::Plus, i))),
                Some((i, '-')) => return Some(Ok((i, TokenType::Minus, i))),
                Some((i, '*')) => return Some(Ok((i, TokenType::Product, i))),
                Some((i, '/')) => return Some(Ok((i, TokenType::Divide, i))),
                Some((i, '%')) => return Some(Ok((i, TokenType::Modulus, i))),
                Some((i, ';')) => return Some(Ok((i, TokenType::SemiColon, i))),
                Some((i, '(')) => return Some(Ok((i, TokenType::OpenParantheses, i))),
                Some((i, ')')) => return Some(Ok((i, TokenType::CloseParantheses, i))),
                Some((i, '"')) => {
                    let l = i;
                    let mut word = String::new();
                    let mut ch = ' ';
                    while let Some((_, c)) = self.chars.next() {
                        ch = c;
                        if c == '"' {
                            break;
                        }
                        word.push(c);
                    }
                    if ch != '"' {
                        panic!("Invalid Literal Closing Quotation Not Found at {}:{}", l, i)
                    }
                    return Some(Ok((i, TokenType::Literal(word), i)));
                }

                Some((i, c)) if c.is_alphabetic() => {
                    let mut word = String::new();
                    word.push(c);
                    while let Some((_, c)) = self.chars.clone().peekable().peek() {
                        if c.is_ascii_alphanumeric() || c == &'_' {
                            word.push(*c);
                            self.chars.next();
                        } else {
                            break;
                        }
                    }

                    match word.as_str() {
                        "pwoli_sadhanam" => return Some(Ok((i, TokenType::Declaration, i))),
                        "address_thada" => return Some(Ok((i, TokenType::InputString, i))),
                        "number_thada" => return Some(Ok((i, TokenType::InputNumber, i))),
                        "dhe_pidicho" => return Some(Ok((i, TokenType::Write, i))),
                        "seriyano_mwone" => return Some(Ok((i, TokenType::If, i))),
                        "seri_allel" => return Some(Ok((i, TokenType::Else, i))),
                        "repeat_adi" => return Some(Ok((i, TokenType::Loop, i))),
                        "inekal_veluthane" => return Some(Ok((i, TokenType::GreaterThan, i))),
                        "um_same_alle" => return Some(Ok((i, TokenType::NotEqual, i))),
                        "inekal_cheruthane" => return Some(Ok((i, TokenType::LessThan, i))),
                        "um_same_aane" => return Some(Ok((i, TokenType::EqualTo, i))),
                        _ => return Some(Ok((i, TokenType::Symbol(word), i))),
                    }
                }

                Some((i, c)) if c.is_digit(10) => {
                    let mut word = String::new();
                    word.push(c);
                    while let Some((_, c)) = self.chars.clone().peekable().peek() {
                        if c == &' '
                            || c == &';'
                            || c == &'+'
                            || c == &'-'
                            || c == &'*'
                            || c == &'/'
                        {
                            break;
                        }
                        word.push(*c);
                        self.chars.next();
                    }

                    if let Ok(number) = word.parse() {
                        return Some(Ok((i, TokenType::Number(number), i)));
                    } else {
                        panic!("Invalid literal {} at {}", word, i);
                    }
                }
                Some((i, c)) => {
                    panic!("Oh no unknown token {} at {} ", c, i);
                }
                None => return None,
            }
        }
    }
}
