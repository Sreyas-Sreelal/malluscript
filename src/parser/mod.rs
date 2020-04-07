mod grammar;
use crate::executor::ast;
use crate::lexer::Lexer;
pub use grammar::SourceUnitParser;
use lalrpop_util::ParseError;

pub fn parse<'a>(src: &'a str, tokens: Lexer<'a>) -> ast::SourceUnit<'a> {
    match grammar::SourceUnitParser::new().parse(&src, tokens) {
        Ok(parsed) => parsed,
        Err(err) => {
            match err {
                ParseError::InvalidToken { location } => {
                    println!("Invalid Token {}", &src[location..location + 1])
                }

                ParseError::UnrecognizedToken {
                    token: (l, token, r),
                    expected,
                } => println!(
                    "{}\nUnrecognised token `{}` expected `{}` ",
                    &src[l..r + 1].trim(),
                    token,
                    expected.join(", ")
                ),

                ParseError::User { error } => println!("Unexpected error {}", error.to_string()),
                ParseError::ExtraToken { token } => println!(
                    "{}\nExtra token `{}' encountered",
                    &src[token.0..token.2 + 1].trim(),
                    token.1
                ),
                ParseError::UnrecognizedEOF {
                    location: _,
                    expected,
                } => println!("Unexpected end of file, expecting {}", expected.join(", ")),
            }
            std::process::exit(1);
        }
    }
}
