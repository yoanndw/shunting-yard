use crate::tokens::Token::{self, *};

pub fn eval_rpn(tokens: Vec<Token>) -> i32 {
    let mut stack = Vec::new();

    for tok in tokens {
        if let Number(nb) = tok {
            stack.push(nb);
        } else {
            let rhs = stack.pop().unwrap();
            let lhs = stack.pop().unwrap();

            let res = match tok {
                Plus => lhs + rhs,
                Minus => lhs - rhs,
                Mul => lhs * rhs,
                Div => lhs / rhs,
                _ => 0,
            };

            stack.push(res);
        }
    }

    *stack.first().unwrap()
}
