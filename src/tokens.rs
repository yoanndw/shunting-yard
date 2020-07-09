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
    number_buf: String,
    on_number: bool,
    tokens: Vec<Token>,
}

impl<'a> Tokenizer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            input,
            number_buf: String::new(),
            on_number: false,
            tokens: Vec::new(),
        }
    }

    fn insert_number(&mut self) {
        let i: i32 = self.number_buf.parse().unwrap();
        self.tokens.push(Token::Number(i));

        self.number_buf.clear();
    }

    pub fn tokenize(&mut self) {
        for c in self.input.chars() {
            if c.is_numeric() {
                // If was not on number => new number => reset buffer
                if !self.on_number {
                    self.number_buf.clear();
                }

                self.on_number = true;

                self.number_buf.push(c);
            } else {
                self.on_number = false;

                // If there is a number in the buffer => insert it
                if !self.number_buf.is_empty() {
                    self.insert_number();
                }

                match c {
                    '+' => self.tokens.push(Token::Plus),
                    '-' => self.tokens.push(Token::Minus),
                    '*' => self.tokens.push(Token::Mul),
                    '/' => self.tokens.push(Token::Div),
                    _ => {}
                }
            }
        }

        // After reached all chars, a self.number_buf might remains
        if !self.number_buf.is_empty() {
            self.insert_number();
        }
    }

    pub fn tokens(&self) -> &Vec<Token> {
        &self.tokens
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

#[cfg(test)]
mod tokenizer_test {
    use super::*;

    #[test]
    fn sum() {
        let mut tokenizer = Tokenizer::new("1+2");
        tokenizer.tokenize();
        let t = tokenizer.tokens();

        assert_eq!(t, &vec![Token::Number(1), Token::Plus, Token::Number(2)]);
    }
}
