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

pub struct Tokenizer<'a> {
    input: &'a str,
}

impl<'a> Tokenizer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self { input }
    }

    pub fn tokenize(&self) -> Vec<Token> {
        let mut tokens = Vec::new();

        let mut number = String::new();
        let mut on_number = false;
        for c in self.input.chars() {
            if c.is_numeric() {
                // If was not on number => new number => reset buffer
                if !on_number {
                    number.clear();
                }

                on_number = true;

                number.push(c);
            } else {
                on_number = false;

                // If there is a number in the buffer => insert it
                if !number.is_empty() {
                    let i: i32 = number.parse().unwrap();
                    tokens.push(Token::Number(i));

                    number.clear();
                }

                match c {
                    '+' => tokens.push(Token::Plus),
                    '-' => tokens.push(Token::Minus),
                    '*' => tokens.push(Token::Mul),
                    '/' => tokens.push(Token::Div),
                    _ => {}
                }
            }
        }

        // After reached all chars, a number might remains
        if !number.is_empty() {
            let i: i32 = number.parse().unwrap();
            tokens.push(Token::Number(i));

            number.clear();
        }

        tokens
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
