pub fn to_postfix(infix: Vec<&str>) -> Vec<&str> {
    let mut stack: Vec<&str> = Vec::new();
    let mut res: Vec<&str> = Vec::new();

    for ch in infix {
        if ch == "(" {
            stack.push(ch);
        } else if ch == ")" {
            while let Some(sym) = stack.pop() {
                if sym == "(" {
                    break;
                }
                res.push(sym as &str);
            }
        } else if is_operator(ch) {
            while let Some(&sym) = stack.last() {
                if precedence(sym) >= precedence(ch) {
                    if let Some(val) = stack.pop() {
                        res.push(val);
                    }
                } else {
                    break;
                }
            }
            stack.push(ch);
        } else {
            res.push(ch);
        }
    }
    while let Some(ch) = stack.pop() {
        res.push(ch);
    }

    res
}

pub fn eval_postfix(postfix: &[&str]) -> f64 {
    let mut stack: Vec<f64> = Vec::new();

    for &ch in postfix {
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
