use crate::tokens::*;

pub fn shunting_yard(tokens: Vec<Token>) -> Vec<Token> {
    let mut op_stack: Vec<Token> = Vec::new();
    let mut output = Vec::new();

    for tok in &tokens {
        if tok.is_op() {
            if let Some(last) = op_stack.last() {
                // Minus must be just after its 2 operands
                if matches!(last, Token::Minus) {
                    if let Some(top) = op_stack.pop() {
                        output.push(top);
                    }
                } else if last.prior(tok) {
                    // Pop all the op stack into output
                    while let Some(top) = op_stack.pop() {
                        output.push(top);
                    }
                }
            }

            op_stack.push(*tok);
        } else {
            output.push(*tok);
        }
    }

    while !op_stack.is_empty() {
        output.push(op_stack.pop().unwrap());
    }

    output
}

#[cfg(test)]
mod test {
    use super::Token::{self, *};
    use super::*;

    #[test]
    fn addition() {
        assert_eq!(
            shunting_yard(vec![Number(1), Plus, Number(2)]),
            vec![Number(1), Number(2), Plus]
        );
    }

    #[test]
    fn mul_op_precedence() {
        assert_eq!(
            shunting_yard(vec![Number(23), Plus, Number(12), Mul, Number(2)]),
            vec![Number(23), Number(12), Number(2), Mul, Plus]
        );

        assert_eq!(
            shunting_yard(vec![Number(1), Mul, Number(2), Minus, Number(1)]),
            vec![Number(1), Number(2), Mul, Number(1), Minus]
        );
    }

    #[test]
    fn minus_after_two_operands() {
        assert_eq!(
            shunting_yard(vec![Number(1), Minus, Number(4), Plus, Number(1)]),
            vec![Number(1), Number(4), Minus, Number(1), Plus]
        );
    }
}
