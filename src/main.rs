mod parse_eval;
mod rpn;
mod shunting_yard;
mod tokens;

use parse_eval::eval;
use rpn::eval_rpn;
use shunting_yard::shunting_yard;
use tokens::*;

fn main() {
    println!("{}", eval("125 + 89  * 897    /8     +5"));
}
