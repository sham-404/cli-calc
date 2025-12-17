mod eval;

use eval::eval_postfix;
use eval::to_postfix;
use eval::tokenize;

fn main() {
    let a: Vec<String> = vec!["23", "+", "5", "/", "(", "45.2", "-", "90", ")"]
        .iter()
        .map(|ch| ch.to_string())
        .collect();

    println!("{:?}", to_postfix(&a));
    println!("{}", eval_postfix(&to_postfix(&a)));
    println!("{}", eval_postfix(&to_postfix(&tokenize("4.34+5"))));
}
