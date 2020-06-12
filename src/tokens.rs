#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Token {
    Number(i32),
    Plus,
    Minus,
    Mul,
    Div,
}

impl Token {
    pub fn is_op(&self) -> bool {
        matches!(self, Token::Plus | Token::Minus | Token::Mul | Token::Div)
    }

    pub fn priority(&self) -> usize {
        use Token::*;

        match self {
            Plus | Minus => 1,
            Mul | Div => 2,
            _ => 0,
        }
    }

    pub fn prior(&self, other: &Token) -> bool {
        self.priority() >= other.priority()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn eq() {
        assert_eq!(Token::Div, Token::Div);
    }
}
