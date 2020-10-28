use std::fmt;

#[derive(Debug)]
pub enum RunTimeErrors {
    UndefinedSymbol(String),
    SymbolAlreadyDefined(String),
    UnInitialzedData(String),
    InvalidAssignment,
    DivisionByZero,
    IncompatibleOperation,
    InvalidExpression,
    InvalidNumberInput,
    ErrorReadingStdin,
    InvalidFunctionDeclaration,
    ArgumentCountMismatch,
}

impl fmt::Display for RunTimeErrors {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RunTimeErrors::UndefinedSymbol(symbol) => {
                write!(f, "[Error]: Undefined symbol {}", symbol)
            }
            RunTimeErrors::SymbolAlreadyDefined(symbol) => {
                write!(f, "[Error]: Symbol {} already defined", symbol)
            }
            RunTimeErrors::UnInitialzedData(data) => write!(f, "Uninitialised variable {}", data),
            RunTimeErrors::DivisionByZero => write!(f, "[Error]: Division by Zero"),
            RunTimeErrors::IncompatibleOperation => {
                write!(f, "[Error]: Incompatible operation between datatypes")
            }
            RunTimeErrors::InvalidExpression => write!(f, "[Error]: Invalid expression"),
            RunTimeErrors::InvalidAssignment => write!(f, "[Error]: Invalid Assignment"),
            RunTimeErrors::InvalidNumberInput => write!(
                f,
                "[Error]: Invalid integer data has been provided as input"
            ),
            RunTimeErrors::ErrorReadingStdin => write!(f, "[Error]: Cannot read input"),
            RunTimeErrors::InvalidFunctionDeclaration => {
                write!(f, "[Error]: Invalid function declaration")
            }
            RunTimeErrors::ArgumentCountMismatch => write!(
                f,
                "[Error]: Argument number doesnot match paramteters number"
            ),
        }
    }
}

pub fn raise_error(error: RunTimeErrors) -> ! {
    println!("{}", error);
    std::process::exit(0)
}
