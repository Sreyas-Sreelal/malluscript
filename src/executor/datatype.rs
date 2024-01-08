use crate::executor::error::{raise_error, RunTimeErrors};
use std::fmt;
use std::ops;

macro_rules! safe_operation {
    ($l:ident + $r:ident) => {
        if let Some(result) = $l.checked_add($r) {
            result
        } else {
            raise_error(RunTimeErrors::IntegerOverFlow)
        }
    };
    ($l:ident - $r:ident) => {
        if let Some(result) = $l.checked_sub($r) {
            result
        } else {
            raise_error(RunTimeErrors::IntegerOverFlow)
        }
    };
    ($l:ident * $r:ident) => {
        if let Some(result) = $l.checked_mul($r) {
            result
        } else {
            raise_error(RunTimeErrors::IntegerOverFlow)
        }
    };
    ($l:ident / $r:ident) => {
        if let Some(result) = $l.checked_div($r) {
            result
        } else {
            raise_error(RunTimeErrors::IntegerOverFlow)
        }
    };
    ($l:ident % $r:ident) => {
        if let Some(result) = $l.checked_rem($r) {
            result
        } else {
            raise_error(RunTimeErrors::IntegerOverFlow)
        }
    };
}

#[derive(Debug, PartialEq, Clone, PartialOrd)]
pub enum DataTypes {
    String(String),
    Integer(i64),
    Bool(bool),
    Float(f64),
    List(Vec<DataTypes>),
    Ref((i64, usize)),
    Unknown,
}

impl fmt::Display for DataTypes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            DataTypes::String(s) => write!(f, "{}", literal_eval(s)),
            DataTypes::Integer(i) => write!(f, "{}", i),
            DataTypes::Float(n) => write!(f, "{}", n),
            DataTypes::List(list) => {
                let mut string = String::from("[");
                let mut iter = list.iter().peekable();
                while let Some(x) = iter.next() {
                    if let DataTypes::String(_) = x {
                        string += &("\"".to_owned() + &x.to_string() + "\"");
                    } else {
                        string += &x.to_string();
                    }

                    if iter.peek().is_some() {
                        string += ",";
                    }
                }
                string += "]";
                write!(f, "{}", string)
            }
            _ => write!(f, "<Garbage:UnintialisedMemorySpace>"),
        }
    }
}

impl ops::Add for DataTypes {
    type Output = Self;
    fn add(self, rhs: DataTypes) -> Self {
        match (self, rhs) {
            (DataTypes::Integer(l), DataTypes::Integer(r)) => {
                DataTypes::Integer(safe_operation!(l + r))
            }
            (DataTypes::Float(l), DataTypes::Float(r)) => DataTypes::Float(l + r),
            (DataTypes::Float(l), DataTypes::Integer(r)) => DataTypes::Float(l + r as f64),
            (DataTypes::Integer(l), DataTypes::Float(r)) => DataTypes::Float(l as f64 + r),
            (DataTypes::Integer(l), DataTypes::String(r)) => DataTypes::String(l.to_string() + &r),
            (DataTypes::String(l), DataTypes::Integer(r)) => DataTypes::String(l + &r.to_string()),
            (DataTypes::Float(l), DataTypes::String(r)) => DataTypes::String(l.to_string() + &r),
            (DataTypes::String(l), DataTypes::Float(r)) => DataTypes::String(l + &r.to_string()),
            (DataTypes::String(l), DataTypes::String(r)) => DataTypes::String(l + &r),
            (DataTypes::List(mut l), r) => {
                l.push(r);
                DataTypes::List(l)
            }
            _ => raise_error(RunTimeErrors::IncompatibleOperation),
        }
    }
}

impl From<bool> for DataTypes {
    fn from(orginal: bool) -> Self {
        DataTypes::Bool(orginal)
    }
}

impl ops::Sub for DataTypes {
    type Output = Self;
    fn sub(self, rhs: DataTypes) -> Self {
        match (self, rhs) {
            (DataTypes::Integer(l), DataTypes::Integer(r)) => {
                DataTypes::Integer(safe_operation!(l - r))
            }
            (DataTypes::Float(l), DataTypes::Float(r)) => DataTypes::Float(l - r),
            (DataTypes::Float(l), DataTypes::Integer(r)) => DataTypes::Float(l - r as f64),
            (DataTypes::Integer(l), DataTypes::Float(r)) => DataTypes::Float(l as f64 - r),
            _ => raise_error(RunTimeErrors::IncompatibleOperation),
        }
    }
}

impl ops::Mul for DataTypes {
    type Output = Self;
    fn mul(self, rhs: DataTypes) -> Self {
        match (self, rhs) {
            (DataTypes::Integer(l), DataTypes::Integer(r)) => {
                DataTypes::Integer(safe_operation!(l * r))
            }
            (DataTypes::Float(l), DataTypes::Float(r)) => DataTypes::Float(l * r),
            (DataTypes::Float(l), DataTypes::Integer(r)) => DataTypes::Float(l * r as f64),
            (DataTypes::Integer(l), DataTypes::Float(r)) => DataTypes::Float(l as f64 * r),
            _ => raise_error(RunTimeErrors::IncompatibleOperation),
        }
    }
}

impl ops::Div for DataTypes {
    type Output = Self;
    fn div(self, rhs: DataTypes) -> Self {
        if rhs == DataTypes::Integer(0) || rhs == DataTypes::Float(0.0) {
            raise_error(RunTimeErrors::DivisionByZero);
        }
        match (self, rhs) {
            (DataTypes::Integer(l), DataTypes::Integer(r)) => {
                DataTypes::Integer(safe_operation!(l / r))
            }
            (DataTypes::Float(l), DataTypes::Float(r)) => DataTypes::Float(l / r),
            (DataTypes::Float(l), DataTypes::Integer(r)) => DataTypes::Float(l / r as f64),
            (DataTypes::Integer(l), DataTypes::Float(r)) => DataTypes::Float(l as f64 / r),
            _ => raise_error(RunTimeErrors::IncompatibleOperation),
        }
    }
}

impl ops::Rem for DataTypes {
    type Output = Self;
    fn rem(self, rhs: DataTypes) -> Self {
        if rhs == DataTypes::Integer(0) || rhs == DataTypes::Float(0.0) {
            raise_error(RunTimeErrors::DivisionByZero);
        }
        match (self, rhs) {
            (DataTypes::Integer(l), DataTypes::Integer(r)) => {
                DataTypes::Integer(safe_operation!(l % r))
            }
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
