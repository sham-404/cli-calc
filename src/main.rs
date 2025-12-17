mod eval;
use eval::eval_postfix;
use eval::to_postfix;

fn main() {
    let a = to_postfix(vec!["23", "+", "5", "/", "(", "45.2", "-", "90", ")"]);

    println!("{:?}", a);
    println!("{}", eval_postfix(&a));
    println!("{}", eval_postfix(&to_postfix(vec!["4", "+", "5"])));
}
