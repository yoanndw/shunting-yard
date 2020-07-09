mod parse_eval;
mod rpn;
mod shunting_yard;
mod tokens;

use std::io::{self, Write};

use parse_eval::eval;
use rpn::eval_rpn;
use shunting_yard::shunting_yard;
use tokens::*;

enum ReplResult {
    ExprResult(i32),
    Quit,
}

fn read_eval() -> ReplResult {
    print!(">>> ");
    io::stdout().flush();

    let mut input = String::new();
    io::stdin().read_line(&mut input);
    let input = input.trim();

    if input == "quit" {
        return ReplResult::Quit;
    }

    ReplResult::ExprResult(eval(input))
}

fn main() {
    println!("Shunting-yard algorithm v1. By yoannd");
    loop {
        let result = read_eval();
        match result {
            ReplResult::ExprResult(res) => println!("{}", res),
            ReplResult::Quit => break,
        }
    }
}
