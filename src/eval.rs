use crate::tokens::Token;

pub fn lexer(input: &str) -> Result<Vec<Token>, String> {
    let mut token_arr: Vec<Token> = Vec::new();
    let mut num: String = String::new();

    for ch in input.chars() {
        if ch.is_numeric() || ch == '.' {
            num.push(ch);
        } else {
            if !num.is_empty() {
                token_arr.push(Token::Number(to_num(&num)?));
                num.clear();
            }
            token_arr.push(Token::match_symbol(ch)?);
        }
    }
    if !num.is_empty() {
        token_arr.push(Token::Number(to_num(&num)?));
    }

    Ok(token_arr)
}

pub fn parser(tokens: &[Token]) -> Result<Vec<Token>, String> {
    let mut res: Vec<Token> = Vec::new();
    let mut append_minus: bool = false;

    for token in tokens {
        // Handling unary '-' (Token::Minus)
        if append_minus {
            append_minus = false;
            if !token.is_num() {
                return Err("Needed valid number after unary minus '-'".to_string());
            }

            let n: f64 = match token {
                &Token::Number(val) => val,
                _ => return Err("Unexpected Error occured at parser()!".to_string()),
            };

            res.push(Token::Number(-n));
            continue;
        }

        if token.is_operator() {
            if matches!(token, Token::Minus) {
                let last_item = res.last();

                match last_item {
                    Some(item) => {
                        if !matches!(item, Token::RParan | Token::Number(_)) {
                            append_minus = true;
                            continue;
                        }
                    }
                    None => {
                        append_minus = true;
                        continue;
                    }
                }
            } else if matches!(token, Token::Plus) {
                let last_item = res.last();

                match last_item {
                    Some(item) => {
                        if !matches!(item, Token::RParan | Token::Number(_)) {
                            continue;
                        }
                    }
                    None => continue,
                }
            }
        }

        if matches!(token, Token::LParan) {
            let last_item = res.last();

            match last_item {
                Some(item) => {
                    if matches!(item, Token::Number(_)) {
                        res.push(Token::Mul);
                    }
                }
                None => {}
            }
        }

        if matches!(token, Token::RParan) {
            let last_item = res.last();

            match last_item {
                Some(item) => {
                    if item.is_operator() {
                        return Err(format!("Invalid Syntax! Operator before ')'"));
                    }
                }
                None => return Err("Invalid Syntax!".to_string()),
            }
        }

        if matches!(token, Token::Number(_)) {
            let last_item = res.last();

            match last_item {
                Some(item) => {
                    if matches!(item, Token::RParan) {
                        res.push(Token::Mul);
                    }
                }
                None => {}
            }
        }

        res.push(token.clone());
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
