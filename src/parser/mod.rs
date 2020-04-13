#![allow(clippy::all)]
mod grammar;
use crate::executor::ast;
use crate::lexer::Lexer;
pub use grammar::SourceUnitParser;
use lalrpop_util::ParseError;

#[cfg(test)]
mod test;

pub fn parse<'a>(src: &'a str, tokens: &mut Lexer<'a>) -> Result<ast::SourceUnit<'a>, String> {
    match grammar::SourceUnitParser::new().parse(&src, tokens) {
        Ok(parsed) => Ok(parsed),
        Err(err) => match err {
            ParseError::InvalidToken { location } => {
                Err(format!("Invalid Token {}", &src[location..=location]))
            }

            ParseError::UnrecognizedToken {
                token: (l, token, r),
                expected,
            } => Err(format!(
                "{}\nUnrecognised token `{}` expected `{}` ",
                &src[l..r].trim(),
                token,
                expected.join(", ")
            )),

            ParseError::User { error } => Err(format!("Unexpected error {}", error.to_string())),
            ParseError::ExtraToken { token } => Err(format!(
                "{}\nExtra token `{}' encountered",
                &src[token.0..=token.2].trim(),
                token.1
            )),
            ParseError::UnrecognizedEOF {
                location: _,
                expected,
            } => Err(format!(
                "Unexpected end of file, expecting {}",
                expected.join(", ")
            )),
        },
    }
}
