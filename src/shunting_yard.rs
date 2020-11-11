use crate::tokens::*;

pub fn shunting_yard(tokens: Vec<Token>) -> Vec<Token> {
    let mut op_stack: Vec<Token> = Vec::new();
    let mut output = Vec::new();

    for tok in &tokens {
        if tok.is_op() {
            // Pop while top of the op_stack is taking precedence to `tok`
            while let Some(top) = op_stack.last() {
                if top.prior(tok) {
                    output.push(op_stack.pop().unwrap());
                } else {
                    break;
                }
            }

            op_stack.push(*tok);
        } else if *tok == Token::OpenBrace {
            op_stack.push(*tok);
        } else if *tok == Token::CloseBrace {
            // Pop while top != "("
            // Pop anyway
            while let Some(popped) = op_stack.pop() {
                //let popped = op_stack.pop().unwrap();

                // If operator => insert into output
                if popped != Token::OpenBrace && popped != Token::CloseBrace {
                    output.push(popped);
                }

                if popped == Token::OpenBrace {
                    break;
                }
            }
        } else {
            output.push(*tok);
        }
    }

    // Pop remaining operators
    while !op_stack.is_empty() {
        output.push(op_stack.pop().unwrap());
    }

    output
}

#[cfg(test)]
mod test {
    use super::Token::*;
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

    #[test]
    fn div() {
        assert_eq!(
            shunting_yard(vec![Number(21), Div, Number(4)]),
            vec![Number(21), Number(4), Div]
        );

        assert_eq!(
            shunting_yard(vec![Number(1), Div, Number(4), Div, Number(2)]),
            vec![Number(1), Number(4), Div, Number(2), Div]
        );

        assert_eq!(
            shunting_yard(vec![Number(1), Plus, Number(4), Div, Number(2)]),
            vec![Number(1), Number(4), Number(2), Div, Plus]
        );
    }
}
