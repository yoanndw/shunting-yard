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
}
