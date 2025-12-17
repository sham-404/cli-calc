use std::mem;

pub fn to_postfix(infix: Vec<String>) -> Vec<String> {
    let mut stack: Vec<&str> = Vec::new();
    let mut res: Vec<String> = Vec::new();

    for item in &infix {
        let ch = item.as_str();
        if ch == "(" {
            stack.push(ch);
        } else if ch == ")" {
            while let Some(sym) = stack.pop() {
                if sym == "(" {
                    break;
                }
                res.push(sym.to_string());
            }
        } else if is_operator(ch) {
            while let Some(&sym) = stack.last() {
                if precedence(sym) >= precedence(ch) {
                    if let Some(val) = stack.pop() {
                        res.push(val.to_string());
                    }
                } else {
                    break;
                }
            }
            stack.push(ch);
        } else {
            res.push(ch.to_string());
        }
    }
    while let Some(ch) = stack.pop() {
        res.push(ch.to_string());
    }

    res
}

pub fn eval_postfix(postfix: Vec<String>) -> f64 {
    let mut stack: Vec<f64> = Vec::new();

    for ch in &postfix {
        if !is_operator(ch) {
            stack.push(ch.parse::<f64>().unwrap());
        } else {
            let a = stack.pop().unwrap();
            let b = stack.pop().unwrap();

            stack.push(compute(a, b, ch));
        }
    }
    stack.pop().unwrap()
}

pub fn tokenize(input: &str) -> Vec<String> {
    let mut token: Vec<String> = Vec::new();
    let mut num: String = String::new();

    for ch in input.chars() {
        if ch.is_numeric() || ch == '.' {
            num.push(ch);
        } else {
            token.push(mem::take(&mut num));
            token.push(ch.to_string());
        }
    }
    if !num.is_empty() {
        token.push(num);
    }

    token
}

fn compute(a: f64, b: f64, ch: &str) -> f64 {
    match ch {
        "^" => b.powf(a),
        "*" => b * a,
        "/" => b / a,
        "+" => b + a,
        "-" => b - a,
        _ => 0.0,
    }
}

fn precedence(c: &str) -> i16 {
    match c {
        "^" => 5,
        "/" | "*" => 3,
        "+" | "-" => 1,
        _ => 0,
    }
}

fn is_operator(op: &str) -> bool {
    matches!(op, "+" | "-" | "*" | "/" | "^")
}
// a + b * (c / d)
