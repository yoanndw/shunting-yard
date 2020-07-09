use crate::{rpn::eval_rpn, shunting_yard::shunting_yard, tokens::tokenize};

pub fn eval(input: &str) -> i32 {
    let tokens = tokenize(input);
    let rpn = shunting_yard(tokens);

    eval_rpn(rpn)
}
