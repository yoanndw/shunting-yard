mod shunting_yard;
mod tokens;

use shunting_yard::shunting_yard;
use tokens::Token::*;

fn main() {
    // 1 + 2 * 3 - 4
    // [Number(1), Number(2), Number(3), Number(4), Minus, Mul, Plus]
    let input = vec![Number(1), Plus, Number(2), Mul, Number(3), Minus, Number(4)];
    let output = shunting_yard(input);

    println!("{:?}", output);
}
