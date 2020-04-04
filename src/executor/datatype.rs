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
            DataTypes::String(s) => write!(f, "{}", s),
            DataTypes::Integer(i) => write!(f, "{}", i),
            _ => write!(f, "<Garbage:UnintialisedMemorySpace>"),
        }
    }
}

impl std::ops::Add for DataTypes {
    type Output = Self;
    fn add(self, rhs: DataTypes) -> Self {
        match (rhs, self) {
            (DataTypes::Integer(r), DataTypes::Integer(l)) => DataTypes::Integer(r + l),
            _ => panic!("Unhandled addition of datatypes"),
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
        match (rhs, self) {
            (DataTypes::Integer(r), DataTypes::Integer(l)) => DataTypes::Integer(r - l),
            _ => panic!("Unhandled subraction of datatypes"),
        }
    }
}

impl std::ops::Mul for DataTypes {
    type Output = Self;
    fn mul(self, rhs: DataTypes) -> Self {
        match (rhs, self) {
            (DataTypes::Integer(r), DataTypes::Integer(l)) => DataTypes::Integer(r * l),
            _ => panic!("Unhandled multiplication of datatypes"),
        }
    }
}

impl std::ops::Div for DataTypes {
    type Output = Self;
    fn div(self, rhs: DataTypes) -> Self {
        match (rhs, self) {
            (DataTypes::Integer(r), DataTypes::Integer(l)) => DataTypes::Integer(r / l),
            _ => panic!("Unhandled division of datatypes"),
        }
    }
}