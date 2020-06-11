mod tokens;
use tokens::Token::{self, *};

fn shunting_yard(tokens: Vec<Token>) -> Vec<Token> {
    let mut op_stack: Vec<Token> = Vec::new();
    let mut output = Vec::new();

    for tok in &tokens {
        if tok.is_op() {
            if let Some(last) = op_stack.last() {
                if last.prior(tok) {
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

fn main() {
    // 1 + 2 * 3 - 4
    // [Number(1), Number(2), Number(3), Number(4), Minus, Mul, Plus]
    let input = vec![Number(1), Plus, Number(2), Mul, Number(3), Minus, Number(4)];
    let output = shunting_yard(input);

    println!("{:?}", output);
}
