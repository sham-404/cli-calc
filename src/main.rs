mod eval;
mod tokens;

use crate::eval::{eval_postfix, lexer, to_postfix};

fn main() {
    let input = "3-(2)";
    println!("Input: {input:?}");

    let lex = lexer(input).unwrap();
    println!("lex: {lex:?}");

    let postfix = to_postfix(&lex);
    println!("Postfix: {postfix:?}");

    let res = eval_postfix(&postfix);
    println!("Result: {res}");
}
