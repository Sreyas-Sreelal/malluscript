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
    keywords:Keywords<'input> ,
    src:&'input str,
}

impl<'input> Lexer<'input> {
    pub fn new(input: &'input str) -> Self {
        Lexer {
            chars: input.char_indices(),
            keywords:Keywords::new(),
            src:input,
        }
    }
    
    fn is_operator(&self,c:&char) -> bool {
        c == &' ' || c == &';'|| c == &'+'|| c == &'-'|| c == &'*'|| c == &'/'
    }

    fn is_valid_name(&self,c:&char) -> bool {
        c.is_ascii_alphanumeric() || c == &'_'
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum LexicalError {
    //TODO:add custom lexical errors instead of panicking
}

pub type Spanned<TokenType, Loc, Error> = Result<(Loc, TokenType, Loc), Error>;

impl<'input> Iterator for Lexer<'input> {
    type Item = Spanned<TokenType<'input> , usize, LexicalError>;

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
                    let (start, mut end) = (i+1,0);
                    let mut ch = ' ';
                    while let Some((_, c)) = self.chars.next() {
                        ch = c;
                        if c == '"' {
                            break;
                        }
                        end +=1;
                    }
                    end = start+end;
                    if ch != '"' {
                        panic!("Invalid Literal Closing Quotation Not Found at {}:{}", end, i)
                    }
                    //println!("literal {}",&self.src[i..i+length+1]);
                    return Some(Ok((i, TokenType::Literal(&self.src[start..end]), i)));
                }

                Some((i, c)) if c.is_alphabetic() => {
                    let (start, mut end) = (i,0);

                    while let Some((_, c)) = self.chars.clone().peekable().peek() {
                        if self.is_valid_name(c) {
                            end += 1;
                            self.chars.next();
                        } else {
                            break;
                        }
                    }
                    end = start+end;
                    //println!("check symbol keyword{}",&self.src[i..i+length+1]);
                    if let Some(keyword) = &self.keywords.list.get(&self.src[start..end+1]) {
                        return Some(Ok((i,(**keyword).clone(),i)));
                    } else {
                        return Some(Ok((i, TokenType::Symbol(&self.src[start..end+1]), i)));
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

