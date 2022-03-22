#[allow(clippy::all)]
#[cfg_attr(rustfmt, rustfmt_skip)]
mod grammar;

use crate::executor::ast;
use crate::lexer::tokens::TokenType;
use crate::lexer::Lexer;
use lalrpop_util::ParseError;
use std::collections::HashMap;

#[cfg(test)]
mod test;

pub fn parse<'a>(src: &'a str, mut tokens: &mut Lexer<'a>) -> Result<ast::SourceUnit, String> {
    match grammar::SourceUnitParser::new().parse(src, &mut tokens) {
        Ok(parsed) => Ok(parsed),
        Err(err) => match err {
            ParseError::InvalidToken { location } => {
                Err(format!("Invalid Token {}", &src[location..=location]))
            }

            ParseError::UnrecognizedToken {
                token: (l, TokenType::Literal(token), r),
                expected,
            } => Err(format!(
                "{}\nUnrecognised token `{}` expected `{}` ",
                &src[l..r].trim(),
                tokens.literal_table[&token],
                expected.join(", ")
            )),
            ParseError::UnrecognizedToken {
                token: (l, TokenType::Symbol(token), r),
                expected,
            } => Err(format!(
                "{}\nUnrecognised token `{}` expected `{}` ",
                &src[l..r].trim(),
                get_symbol_name(&tokens.symbol_lookup, token),
                expected.join(", ")
            )),
            ParseError::UnrecognizedToken {
                token: (l, token, r),
                expected,
            } => Err(format!(
                "{}\nUnrecognised token `{}` expected `{}` ",
                &src[l..r].trim(),
                token,
                expected.join(", ")
            )),

            ParseError::User { error } => Err(format!("Unexpected error {}", error)),
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

fn get_symbol_name(table: &HashMap<String, usize>, address: usize) -> String {
    if let Some(name) = table.iter().find_map(|(key, &val)| {
        if val == address {
            Some(key.to_string())
        } else {
            None
        }
    }) {
        name
    } else {
        "<invalid-entry-in-symbol-table>".to_string()
    }
}
