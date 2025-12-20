mod eval;
mod tokens;

use std::env;

use crate::eval::{eval_postfix, lexer, parser, to_postfix};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        println!("Example usage: calc <mode> <expression> <optional-parameters>");
        return;
    }

    match args[1].as_str() {
        "-c" => match args.get(3) {
            Some(param) => calculate(&args[2], &param),
            None => calculate(&args[2], &"null".to_string()),
        },
        _ => println!("Unexpected bug"),
    }
}

fn calculate(exp: &String, param: &str) {
    let input = exp;
    let lex = match lexer(input) {
        Ok(lex) => lex,
        Err(err) => return eprintln!("{err}"),
    };
    let parsed = match parser(&lex) {
        Ok(parse) => parse,
        Err(err) => return eprintln!("{err}"),
    };
    let postfix = to_postfix(&parsed);
    let res = eval_postfix(&postfix);

    println!("{res}");
    println!();

    match param {
        "debug" => {
            println!("Input: {input:?}");
            println!("lex: {lex:?}");
            println!("parser: {:?}", parsed);
            println!("Postfix: {postfix:?}");
            println!("Result: {res}");
        }
        "null" => return,
        _ => println!("Invalid Parameter"),
    }
}
