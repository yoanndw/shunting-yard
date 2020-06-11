mod rpn;
mod shunting_yard;
mod tokens;

use rpn::eval_rpn;
use shunting_yard::shunting_yard;
use tokens::Token::*;

fn main() {
    // 1 + 2 * 3 - 4
    // [Number(1), Number(2), Number(3), Number(4), Minus, Mul, Plus]
    let input = vec![
        Number(1),
        Minus,
        Number(4),
        Plus,
        Number(2),
        Mul,
        Number(3),
        Mul,
        Number(3),
        Mul,
        Number(3),
    ];
    let output = shunting_yard(input);

    println!("{:?}", output);
    println!("{}", eval_rpn(output));
}
