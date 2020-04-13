mod error;
///
/// Malluscript Lexer
///
mod keywords;
pub mod tokens;

pub use error::LexicalError;
use keywords::Keywords;
use std::str::CharIndices;
use tokens::TokenType;
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct Lexer<'input> {
    chars: CharIndices<'input>,
    keywords: Keywords,
    src: &'input str,
    pub literal_table:HashMap<usize,String>,
    pub symbol_lookup:HashMap<String,usize>,
    lookup_count:usize,
    literal_count:usize,
}

impl<'input> Lexer<'input> {
    pub fn new(input: &'input str) -> Self {
        Lexer {
            chars: input.char_indices(),
            keywords: Keywords::new(),
            src: input,
            literal_table:HashMap::new(),
            symbol_lookup:HashMap::new(),
            literal_count:0,
            lookup_count:0,
        }
    }

    fn is_operator(&self, c: char) -> bool {
        c == ' '
            || c == ';'
            || c == '+'
            || c == '-'
            || c == '*'
            || c == '/'
            || c == '%'
            || c == '\n'
            || c == ')'
            || c == '='
    }

    fn is_valid_name(&self, c: char) -> bool {
        !self.is_operator(c)
    }
}

pub type Spanned<TokenType, Loc, Error> = Result<(Loc, TokenType, Loc), Error>;

impl<'input> Iterator for &mut Lexer<'input> {
    type Item = Spanned<TokenType, usize, LexicalError>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.chars.next() {
                Some((_, ' ')) | Some((_, '\n')) => {
                    continue;
                }
                Some((i, '=')) => return Some(Ok((i, TokenType::Assignment, i + 1))),
                Some((i, '{')) => return Some(Ok((i, TokenType::LeftBrace, i + 1))),
                Some((i, '}')) => return Some(Ok((i, TokenType::RightBrace, i + 1))),
                Some((i, '+')) => return Some(Ok((i, TokenType::Plus, i + 1))),
                Some((i, '-')) => return Some(Ok((i, TokenType::Minus, i + 1))),
                Some((i, '*')) => return Some(Ok((i, TokenType::Product, i + 1))),
                Some((i, '/')) => return Some(Ok((i, TokenType::Divide, i + 1))),
                Some((i, '%')) => return Some(Ok((i, TokenType::Modulo, i + 1))),
                Some((i, ';')) => return Some(Ok((i, TokenType::SemiColon, i + 1))),
                Some((i, '(')) => return Some(Ok((i, TokenType::OpenParantheses, i + 1))),
                Some((i, ')')) => return Some(Ok((i, TokenType::CloseParantheses, i + 1))),
                Some((i, '"')) => {
                    let (start, mut end) = (i + 1, 0);
                    let mut ch = ' ';
                    let mut word = String::new();
                    while let Some((_, c)) = self.chars.next() {
                        ch = c;
                        if c == '"' {
                            break;
                        }
                        word.push(c);
                        end += 1;
                    }
                    end += start;
                    if ch != '"' {
                        return Some(Err(LexicalError::InvalidStringLiteral(i, end)));
                        //panic!(
                        //    "Invalid Literal Closing Quotation Not Found at {}:{}",
                        //    end, i
                        //)
                    }
                    self.literal_count += 1;
                    self.literal_table.insert(self.literal_count,word);
                    return Some(Ok((i, TokenType::Literal(self.literal_count), end)));
                }

                Some((i, c)) if c.is_alphabetic() => {
                    let (start, mut end) = (i, 0);
                    let mut word = String::new();
                    word.push(c);
                    while let Some((_, c)) = self.chars.clone().peekable().peek() {
                        if self.is_valid_name(*c) {
                            end += 1;
                            word.push(*c);
                            self.chars.next();
                        } else {
                            break;
                        }
                    }
                    end += start;
                    //println!("check symbol keyword {}",word);
                    if let Some(keyword) = &self.keywords.list.get(&word) {
                        return Some(Ok((i, (**keyword).clone(), end)));
                    } else if !self.symbol_lookup.contains_key(&word) {
                            self.lookup_count += 1;
                            self.symbol_lookup.insert(word.clone(), self.lookup_count);
                    }
                    return Some(Ok((i, TokenType::Symbol(*(self.symbol_lookup.get(&word).unwrap())), end)));
                }

                Some((i, c)) if c.is_digit(10) => {
                    let mut word = String::new();
                    word.push(c);
                    while let Some((_, c)) = self.chars.clone().peekable().peek() {
                        if self.is_operator(*c) {
                            break;
                        }
                        word.push(*c);
                        self.chars.next();
                    }

                    if let Ok(number) = word.parse() {
                        return Some(Ok((i, TokenType::Number(number), i + word.len())));
                    } else {
                        return Some(Err(LexicalError::InvalidIntegerConstant(
                            i,
                            i + word.len(),
                            word,
                        )));
                    }
                }
                Some((i, _)) => {
                    return Some(Err(LexicalError::UnknownToken(i, i + 1)));
                }
                None => return None,
            }
        }
    }
}
