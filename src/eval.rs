use crate::tokens::Token;

pub fn lexer(input: &str) -> Result<Vec<Token>, String> {
    let mut token: Vec<Token> = Vec::new();
    let mut num: String = String::new();

    for ch in input.chars() {
        if ch.is_numeric() || ch == '.' {
            num.push(ch);
        } else {
            if !num.is_empty() {
                match to_num(&num) {
                    Ok(val) => token.push(Token::Number(val)),
                    Err(err) => eprintln!("{}", err),
                }
                num.clear();
            }
            match Token::match_symbol(ch) {
                Ok(op) => token.push(op),
                Err(err) => eprintln!("{}", err),
            }
        }
    }
    if !num.is_empty() {
        match to_num(&num) {
            Ok(val) => token.push(Token::Number(val)),
            Err(err) => eprintln!("{}", err),
        }
    }

    Ok(token)
}

pub fn parser(tokens: Vec<Token>) -> Result<Vec<Token>, String> {
    let mut res: Vec<Token> = Vec::new();

    for token in tokens {
        if token.is_operator() {
            if let Some(item) = res.last() {
                if !item.is_num() {
                    let sign = res.pop();
                    if matches!(sign.unwrap(), Token::Minus) {}
                    res.push(token);
                }
            }
        }
    }

    Ok(res)
}

pub fn to_postfix(infix: &[Token]) -> Vec<Token> {
    let mut stack: Vec<&Token> = Vec::new();
    let mut res: Vec<Token> = Vec::new();

    for ch in infix {
        if matches!(ch, Token::LParan) {
            stack.push(ch);
        } else if matches!(ch, Token::RParan) {
            while let Some(sym) = stack.pop() {
                if matches!(sym, Token::LParan) {
                    break;
                }
                res.push(sym.clone());
            }
        } else if ch.is_operator() {
            while let Some(&sym) = stack.last() {
                if sym.precedence() >= ch.precedence() {
                    if let Some(val) = stack.pop() {
                        res.push(val.clone());
                    }
                } else {
                    break;
                }
            }
            stack.push(ch);
        } else {
            res.push(ch.clone());
        }
    }
    while let Some(ch) = stack.pop() {
        res.push(ch.clone());
    }

    res
}

pub fn eval_postfix(postfix: &[Token]) -> f64 {
    let mut stack: Vec<Token> = Vec::new();

    for ch in postfix {
        if !ch.is_operator() {
            stack.push(ch.clone());
        } else {
            let a = stack.pop().unwrap();
            let b = stack.pop().unwrap();

            stack.push(compute(&a, &b, ch));
        }
    }
    let ans = match stack.pop().unwrap() {
        Token::Number(n) => n,
        _ => panic!("Unexpected error at eval_postfix()"),
    };

    ans
}

fn compute(a: &Token, b: &Token, ch: &Token) -> Token {
    let (x, y) = match (a, b) {
        (Token::Number(n1), Token::Number(n2)) => (*n1, *n2),
        _ => panic!("Unexpected error at comput()"),
    };

    let res = match ch {
        Token::Pow => y.powf(x),
        Token::Mul => y * x,
        Token::Div => y / x,
        Token::Plus => y + x,
        Token::Minus => y - x,
        _ => 0.0,
    };

    Token::Number(res)
}

fn to_num(n: &str) -> Result<f64, String> {
    n.parse::<f64>().map_err(|_| "Invalid syntax".to_string())
}

// a + b * (c / d)
