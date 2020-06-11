#[derive(Debug, Clone, Copy)]
pub enum Token {
    Number(i32),
    Plus,
    Minus,
    Mul,
    Div,
}

impl Token {
    pub fn is_number(&self) -> bool {
        matches!(self, Token::Number(_))
    }

    pub fn is_op(&self) -> bool {
        matches!(self, Token::Plus | Token::Minus | Token::Mul | Token::Div)
    }

    pub fn prior(&self, other: &Token) -> bool {
        matches!(self, Token::Mul | Token::Div) && matches!(other, Token::Plus | Token::Minus)
    }
}
