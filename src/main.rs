mod eval;
use eval::to_postfix;

fn main() {
    println!(
        "{:?}",
        to_postfix(vec!["23", "+", "5", "/", "(", "45.2", "-", "90", ")"])
    );
}
