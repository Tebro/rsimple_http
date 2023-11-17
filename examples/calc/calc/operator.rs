use std::fmt::{Display, Formatter};

pub enum Operator {
    Plus,
    Minus,
    Multiply,
    Divide,
}

impl Operator {
    pub fn to_string(&self) -> &str {
        match self {
            Operator::Plus => "+",
            Operator::Minus => "-",
            Operator::Multiply => "*",
            Operator::Divide => "/",
        }
    }
}

impl Display for Operator {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_string() {
        assert_eq!(Operator::Plus.to_string(), "+");
    }
}
