mod eval;
use eval::to_postfix;

fn main() {
    println!("{:?}", to_postfix(&"a+(b*c)+d".to_string()));
}
