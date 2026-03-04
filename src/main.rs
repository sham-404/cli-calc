mod eval;
mod tokens;
mod utils;

use std::env;

use crate::eval::{eval_postfix, lexer, parser, to_postfix};
use utils::input;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("");

    if args.len() == 1 {
        println!("Enter the calculations, type quit/exit to finish");
        let mut buffer;

        buffer = input("-> ");
        while buffer != "exit" && buffer != "quit" {
            calculate(&buffer, "null");
            println!();
            buffer = input("-> ");
        }
        return;
    }

    if args.len() < 3 {
        println!("Example usage: calc <mode> <expression> <optional-parameters>");
        return;
    }

    match args[1].as_str() {
        "-c" => match args.get(3) {
            Some(param) => calculate(&args[2], &param),
            None => calculate(&args[2], "null"),
        },
        _ => eprintln!("Invalid mode"),
    }
}

fn calculate(exp: &String, param: &str) {
    let input = exp.replace(" ", "");
    let lex = match lexer(&input) {
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
