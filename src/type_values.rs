use std::cmp::Ordering;
use std::fmt::{Display, Formatter};
use std::ops::{Add, AddAssign, Div, Mul, Sub, SubAssign};

#[derive(Debug, Clone)]
pub enum Type {
    Int(i64),
    Str(String),
}
impl Type {
    pub fn is_zero(&self) -> bool {
        matches!(self, Type::Int(0))
    }
}

impl Display for Type {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Type::Int(n) => write!(f, "{}", n),
            Type::Str(s) => write!(f, "{}", s),
        }
    }
}

impl AddAssign for Type {
    fn add_assign(&mut self, other: Type) {
        *self = self.clone() + other;
    }
}

impl SubAssign for Type {
    fn sub_assign(&mut self, other: Type) {
        *self = self.clone() - other;
    }
}
impl PartialOrd for Type {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Type::Int(a), Type::Int(b)) => a.partial_cmp(b),
            (Type::Str(a), Type::Str(b)) => a.partial_cmp(b),
            _ => None,
        }
    }
}

impl PartialEq for Type {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Type::Int(a), Type::Int(b)) => a == b,
            (Type::Str(a), Type::Str(b)) => a == b,
            _ => false,
        }
    }
}

impl Div for Type {
    type Output = Type;
    fn div(self, rhs: Type) -> Self::Output {
        match (self, rhs) {
            (Type::Int(a), Type::Int(b)) => Type::Int(a / b),
            _ => panic!("Cannot divide non-ints"),
        }
    }
}

impl Add for Type {
    type Output = Type;
    fn add(self, other: Type) -> Self::Output {
        match (self, other) {
            (Type::Int(a), Type::Int(b)) => Type::Int(a + b),
            (Type::Str(a), Type::Str(b)) => Type::Str(a + &b),
            (Type::Str(a), Type::Int(b)) => Type::Str(a + &b.to_string()),
            (Type::Int(a), Type::Str(b)) => Type::Str(a.to_string() + &b),
        }
    }
}
impl Sub for Type {
    type Output = Type;
    fn sub(self, rhs: Type) -> Self::Output {
        match (self, rhs) {
            (Type::Int(a), Type::Int(b)) => Type::Int(a - b),
            _ => panic!("Cannot subtract non-ints"),
        }
    }
}

impl Mul for Type {
    type Output = Type;
    fn mul(self, rhs: Type) -> Self::Output {
        match (self, rhs) {
            (Type::Int(a), Type::Int(b)) => Type::Int(a * b),
            (Type::Str(a), Type::Int(b)) => Type::Str(a.repeat(b as usize)),
            (Type::Int(a), Type::Str(b)) => Type::Str(b.repeat(a as usize)),
            (Type::Str(a), Type::Str(b)) => panic!("Cannot multiply strings: {} and {}", a, b),
        }
    }
}
