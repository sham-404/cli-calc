use std::collections::HashSet;

pub fn to_postfix(infix: &String) -> Vec<char> {
    let mut stack: Vec<char> = Vec::new();
    let mut res: Vec<char> = Vec::new();
    let operators: HashSet<char> = ['+', '-', '*', '/', '^'].into_iter().collect();

    for ref ch in infix.chars() {
        if ch == &'(' {
            stack.push(*ch);
        } else if ch == &')' {
            while let Some(sym) = stack.pop() {
                if sym == '(' {
                    break;
                }
                res.push(sym);
            }
        } else if operators.contains(ch) {
            while let Some(sym) = stack.last() {
                if precedence(sym) >= precedence(ch) {
                    if let Some(val) = stack.pop() {
                        res.push(val);
                    }
                } else {
                    break;
                }
            }
            stack.push(*ch);
        } else if ch.is_alphabetic() {
            res.push(*ch);
        }
    }
    while !stack.is_empty() {
        res.push(stack.pop().unwrap());
    }

    res
}

fn precedence(c: &char) -> i16 {
    match c {
        '^' => 5,
        '/' => 3,
        '*' => 3,
        '-' => 1,
        '+' => 1,
        _ => 0,
    }
}

// a + b * (c / d)
