mod error;
///
/// Guhiki Lexer
///
mod keywords;
pub mod tokens;

pub use error::LexicalError;
use keywords::Keywords;
use std::str::CharIndices;
use tokens::TokenType;

#[derive(Clone, Debug)]
pub struct Lexer<'input> {
    chars: CharIndices<'input>,
    keywords: Keywords<'input>,
    src: &'input str,
}

impl<'input> Lexer<'input> {
    pub fn new(input: &'input str) -> Self {
        Lexer {
            chars: input.char_indices(),
            keywords: Keywords::new(),
            src: input,
        }
    }

    fn is_operator(&self, c: char) -> bool {
        c == ' '
            || c == ';'
            || c == '+'
            || c == '-'
            || c == '*'
            || c == '/'
            || c == '\n'
            || c == ')'
            || c == '='
    }

    fn is_valid_name(&self, c: char) -> bool {
        c.is_ascii_alphanumeric() || c == '_'
    }
}

pub type Spanned<TokenType, Loc, Error> = Result<(Loc, TokenType, Loc), Error>;

impl<'input> Iterator for Lexer<'input> {
    type Item = Spanned<TokenType<'input>, usize, LexicalError>;

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
                    while let Some((_, c)) = self.chars.next() {
                        ch = c;
                        if c == '"' {
                            break;
                        }
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
                    //println!("literal {}",&self.src[i..i+length+1]);
                    return Some(Ok((i, TokenType::Literal(&self.src[start..end]), end)));
                }

                Some((i, c)) if c.is_alphabetic() => {
                    let (start, mut end) = (i, 0);

                    while let Some((_, c)) = self.chars.clone().peekable().peek() {
                        if self.is_valid_name(*c) {
                            end += 1;
                            self.chars.next();
                        } else {
                            break;
                        }
                    }
                    end += start;
                    //println!("check symbol keyword{}",&self.src[i..i+length+1]);
                    if let Some(keyword) = &self.keywords.list.get(&self.src[start..=end]) {
                        return Some(Ok((i, (**keyword).clone(), end)));
                    } else {
                        return Some(Ok((i, TokenType::Symbol(&self.src[start..=end]), end)));
                    }
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
