mod rpn;
mod shunting_yard;
mod tokens;

use rpn::eval_rpn;
use shunting_yard::shunting_yard;
use tokens::*;

fn main() {
    let tokens = tokenize("1+2");

    println!("{:?}", tokens);
}
