///
/// Guhiki Lexer
///
mod keywords;
pub mod tokens;
use keywords::Keywords;
use tokens::TokenType;
use std::str::CharIndices;

#[derive(Clone)]
pub struct Lexer<'input> {
    chars: CharIndices<'input>,
    keywords:Keywords
}

impl<'input> Lexer<'input> {
    pub fn new(input: &'input str) -> Self {
        Lexer {
            chars: input.char_indices(),
            keywords:Keywords::new()
        }
    }
    
    fn is_operator(&self,c:&char) -> bool {
        c == &' ' || c == &';'|| c == &'+'|| c == &'-'|| c == &'*'|| c == &'/'
    }

    fn is_valid_name(&self,c:&char) -> bool {
        c.is_ascii_alphanumeric() || c == &'_'
    }

    fn literal_eval(&self,data: String) -> String {
        data.replace("\\n", "\n")
    }

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
                    
                    return Some(Ok((i, TokenType::Literal(self.literal_eval(word)), i)));
                }

                Some((i, c)) if c.is_alphabetic() => {
                    let mut word = String::new();
                    word.push(c);
                    while let Some((_, c)) = self.chars.clone().peekable().peek() {
                        if self.is_valid_name(c) {
                            word.push(*c);
                            self.chars.next();
                        } else {
                            break;
                        }
                    }

                    if let Some(keyword) = &self.keywords.list.get(word.as_str()) {
                        return Some(Ok((i,(**keyword).clone(),i)));
                    } else {
                        return Some(Ok((i, TokenType::Symbol(word), i)));
                    }
                    
                }

                Some((i, c)) if c.is_digit(10) => {
                    let mut word = String::new();
                    word.push(c);
                    while let Some((_, c)) = self.chars.clone().peekable().peek() {
                        if self.is_operator(c){
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

