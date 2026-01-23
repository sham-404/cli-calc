use std::io::{self, Write};

pub fn input(prompt: &str) -> String {
    let mut buffer = String::new();

    print!("{}", prompt);
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut buffer).unwrap();

    return buffer.trim().to_string();
}
