use crate::{rpn::eval_rpn, shunting_yard::shunting_yard, tokens::tokenize};

pub fn eval(input: &str) -> i32 {
    let tokens = tokenize(input);
    let rpn = shunting_yard(tokens);

    eval_rpn(rpn)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn sum() {
        assert_eq!(eval("1+2"), 3);
    }

    #[test]
    fn whitespaces() {
        assert_eq!(eval("1      +    5   *8"), 41);

        assert_eq!(
            eval("                   5             *            5       "),
            25
        );
    }

    #[test]
    fn complex_expr() {
        assert_eq!(eval("125 + 89  * 897    /8     +5"), 10_109);
    }

    #[test]
    fn number() {
        assert_eq!(eval("0"), 0);
        assert_eq!(eval("3"), 3);
        assert_eq!(eval("   12   "), 12);
    }
}
