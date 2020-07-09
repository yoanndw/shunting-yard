mod rpn;
mod shunting_yard;
mod tokens;

use rpn::eval_rpn;
use shunting_yard::shunting_yard;
use tokens::*;

fn main() {
    let tokenizer = Tokenizer::new("1 +  2   * 33");
    let tokens = tokenizer.tokenize();

    println!("{:?}", tokens);
}
