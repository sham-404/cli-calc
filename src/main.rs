mod eval;
use eval::eval_postfix;
use eval::to_postfix;

use crate::eval::tokenize;

fn main() {
    let a: Vec<String> = to_postfix(
        vec!["23", "+", "5", "/", "(", "45.2", "-", "90", ")"]
            .iter_mut()
            .map(|ch| ch.to_string())
            .collect(),
    );

    println!("{:?}", a);
    println!("{}", eval_postfix(a));
    println!("{}", eval_postfix(to_postfix(tokenize("4+5"))));
}
