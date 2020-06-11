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
