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

fn insert_number(tokens: &mut Vec<Token>, number_buf: &mut String) {
    let i: i32 = number_buf.parse().unwrap();
    tokens.push(Token::Number(i));

    number_buf.clear();
}

pub fn tokenize(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();

    let mut on_number = false;
    let mut number_buf = String::new();
    for c in input.chars() {
        if c.is_numeric() {
            // If was not on number => new number => reset buffer
            if !on_number {
                number_buf.clear();
            }

            on_number = true;

            number_buf.push(c);
        } else {
            on_number = false;

            // If there is a number in the buffer => insert it
            if !number_buf.is_empty() {
                insert_number(&mut tokens, &mut number_buf);
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

    // After reached all chars, a number_buf might remains
    if !number_buf.is_empty() {
        insert_number(&mut tokens, &mut number_buf);
    }

    tokens
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
    use super::Token::*;
    use super::*;

    #[test]
    fn sum() {
        let t = tokenize("1+2");

        assert_eq!(t, vec![Number(1), Plus, Number(2)]);
    }

    #[test]
    fn whitespaces() {
        assert_eq!(tokenize("1 + 2"), vec![Number(1), Plus, Number(2)]);

        assert_eq!(tokenize("   2 *    8"), vec![Number(2), Mul, Number(8)]);
    }

    #[test]
    fn bigger_number() {
        assert_eq!(tokenize("125"), vec![Number(125)]);

        assert_eq!(tokenize("12+56"), vec![Number(12), Plus, Number(56)]);
    }

    #[test]
    fn complex_expr() {
        assert_eq!(
            tokenize("125 + 89  * 897    /8     +5"),
            vec![
                Number(125),
                Plus,
                Number(89),
                Mul,
                Number(897),
                Div,
                Number(8),
                Plus,
                Number(5)
            ]
        );
    }
}
