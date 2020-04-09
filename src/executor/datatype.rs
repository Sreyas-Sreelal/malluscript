use crate::executor::error::{raise_error, RunTimeErrors};
use std::fmt;

#[derive(Debug, PartialEq, Clone, Eq, PartialOrd)]
pub enum DataTypes {
    String(String),
    Integer(i64),
    Bool(bool),
    Unknown,
}

impl fmt::Display for DataTypes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            DataTypes::String(s) => write!(f, "{}", literal_eval(s)),
            DataTypes::Integer(i) => write!(f, "{}", i),
            _ => write!(f, "<Garbage:UnintialisedMemorySpace>"),
        }
    }
}

impl std::ops::Add for DataTypes {
    type Output = Self;
    fn add(self, rhs: DataTypes) -> Self {
        match (self, rhs) {
            (DataTypes::Integer(l), DataTypes::Integer(r)) => DataTypes::Integer(l + r),
            (DataTypes::Integer(l), DataTypes::String(r)) => DataTypes::String(l.to_string() + &r),
            (DataTypes::String(l), DataTypes::Integer(r)) => DataTypes::String(l + &r.to_string()),
            (DataTypes::String(l), DataTypes::String(r)) => DataTypes::String(l + &r),
            _ => raise_error(RunTimeErrors::IncompatibleOperation),
        }
    }
}

impl From<bool> for DataTypes {
    fn from(orginal: bool) -> Self {
        DataTypes::Bool(orginal)
    }
}

impl std::ops::Sub for DataTypes {
    type Output = Self;
    fn sub(self, rhs: DataTypes) -> Self {
        match (self, rhs) {
            (DataTypes::Integer(l), DataTypes::Integer(r)) => DataTypes::Integer(l - r),
            _ => raise_error(RunTimeErrors::IncompatibleOperation),
        }
    }
}

impl std::ops::Mul for DataTypes {
    type Output = Self;
    fn mul(self, rhs: DataTypes) -> Self {
        match (self, rhs) {
            (DataTypes::Integer(l), DataTypes::Integer(r)) => DataTypes::Integer(l * r),
            _ => raise_error(RunTimeErrors::IncompatibleOperation),
        }
    }
}

impl std::ops::Div for DataTypes {
    type Output = Self;
    fn div(self, rhs: DataTypes) -> Self {
        match (self, rhs) {
            (DataTypes::Integer(l), DataTypes::Integer(r)) => {
                if r != 0 {
                    DataTypes::Integer(l / r)
                } else {
                    raise_error(RunTimeErrors::DivisionByZero);
                }
            }
            _ => raise_error(RunTimeErrors::IncompatibleOperation),
        }
    }
}

impl std::ops::Rem for DataTypes {
    type Output = Self;
    fn rem(self, rhs: DataTypes) -> Self {
        match (self, rhs) {
            (DataTypes::Integer(l), DataTypes::Integer(r)) => {
                if r != 0 {
                    DataTypes::Integer(l % r)
                } else {
                    raise_error(RunTimeErrors::DivisionByZero);
                }
            }
            _ => raise_error(RunTimeErrors::IncompatibleOperation),
        }
    }
}

pub fn to_bool(data: DataTypes) -> bool {
    match data {
        DataTypes::Bool(value) => return value,
        DataTypes::Integer(value) => return value != 0,
        _ => panic!("illegal datatype conversion"),
    }
}

fn literal_eval(data: &str) -> String {
    data.replace("\\n", "\n")
}
