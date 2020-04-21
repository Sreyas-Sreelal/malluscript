use crate::executor::error::{raise_error, RunTimeErrors};
use std::fmt;

#[derive(Debug, PartialEq, Clone, PartialOrd)]
pub enum DataTypes {
    String(String),
    Integer(i64),
    Bool(bool),
    Float(f64),
    Unknown,
}

impl fmt::Display for DataTypes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            DataTypes::String(s) => write!(f, "{}", literal_eval(s)),
            DataTypes::Integer(i) => write!(f, "{}", i),
            DataTypes::Float(n) => write!(f, "{}", n),
            _ => write!(f, "<Garbage:UnintialisedMemorySpace>"),
        }
    }
}

impl std::ops::Add for DataTypes {
    type Output = Self;
    fn add(self, rhs: DataTypes) -> Self {
        match (self, rhs) {
            (DataTypes::Integer(l), DataTypes::Integer(r)) => DataTypes::Integer(l + r),
            (DataTypes::Float(l), DataTypes::Float(r)) => DataTypes::Float(l + r),
            (DataTypes::Float(l), DataTypes::Integer(r)) => DataTypes::Float(l + r as f64),
            (DataTypes::Integer(l), DataTypes::Float(r)) => DataTypes::Float(l as f64 + r),
            (DataTypes::Integer(l), DataTypes::String(r)) => DataTypes::String(l.to_string() + &r),
            (DataTypes::String(l), DataTypes::Integer(r)) => DataTypes::String(l + &r.to_string()),
            (DataTypes::Float(l), DataTypes::String(r)) => DataTypes::String(l.to_string() + &r),
            (DataTypes::String(l), DataTypes::Float(r)) => DataTypes::String(l + &r.to_string()),
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
            (DataTypes::Float(l), DataTypes::Float(r)) => DataTypes::Float(l - r),
            (DataTypes::Float(l), DataTypes::Integer(r)) => DataTypes::Float(l - r as f64),
            (DataTypes::Integer(l), DataTypes::Float(r)) => DataTypes::Float(l as f64 - r),
            _ => raise_error(RunTimeErrors::IncompatibleOperation),
        }
    }
}

impl std::ops::Mul for DataTypes {
    type Output = Self;
    fn mul(self, rhs: DataTypes) -> Self {
        match (self, rhs) {
            (DataTypes::Integer(l), DataTypes::Integer(r)) => DataTypes::Integer(l * r),
            (DataTypes::Float(l), DataTypes::Float(r)) => DataTypes::Float(l * r),
            (DataTypes::Float(l), DataTypes::Integer(r)) => DataTypes::Float(l * r as f64),
            (DataTypes::Integer(l), DataTypes::Float(r)) => DataTypes::Float(l as f64 * r),
            _ => raise_error(RunTimeErrors::IncompatibleOperation),
        }
    }
}

impl std::ops::Div for DataTypes {
    type Output = Self;
    fn div(self, rhs: DataTypes) -> Self {
        if rhs == DataTypes::Integer(0) || rhs == DataTypes::Float(0.0) {
            raise_error(RunTimeErrors::DivisionByZero);
        }
        match (self, rhs) {
            (DataTypes::Integer(l), DataTypes::Integer(r)) => DataTypes::Integer(l / r),
            (DataTypes::Float(l), DataTypes::Float(r)) => DataTypes::Float(l / r),
            (DataTypes::Float(l), DataTypes::Integer(r)) => DataTypes::Float(l / r as f64),
            (DataTypes::Integer(l), DataTypes::Float(r)) => DataTypes::Float(l as f64 / r),
            _ => raise_error(RunTimeErrors::IncompatibleOperation),
        }
    }
}

impl std::ops::Rem for DataTypes {
    type Output = Self;
    fn rem(self, rhs: DataTypes) -> Self {
        if rhs == DataTypes::Integer(0) || rhs == DataTypes::Float(0.0) {
            raise_error(RunTimeErrors::DivisionByZero);
        }
        match (self, rhs) {
            (DataTypes::Integer(l), DataTypes::Integer(r)) => DataTypes::Integer(l % r),
            (DataTypes::Float(l), DataTypes::Float(r)) => DataTypes::Float(l % r),
            (DataTypes::Float(l), DataTypes::Integer(r)) => DataTypes::Float(l % r as f64),
            (DataTypes::Integer(l), DataTypes::Float(r)) => DataTypes::Float(l as f64 % r),
            _ => raise_error(RunTimeErrors::IncompatibleOperation),
        }
    }
}

pub fn to_bool(data: DataTypes) -> bool {
    match data {
        DataTypes::Bool(value) => value,
        DataTypes::Integer(value) => value != 0,
        _ => panic!("illegal datatype conversion"),
    }
}

fn literal_eval(data: &str) -> String {
    data.replace("\\n", "\n")
}
