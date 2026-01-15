#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Number(f64),
    Plus,
    Minus,
    Mul,
    Div,
    Pow,
    LParan,
    RParan,
    Sin(f64),
    Cos(f64),
    Tan(f64),
    Cot(f64),
    Sec(f64),
    Cosec(f64),
}

impl Token {
    pub fn is_operator(&self) -> bool {
        matches!(
            self,
            Token::Pow | Token::Div | Token::Mul | Token::Plus | Token::Minus
        )
    }

    pub fn is_num(&self) -> bool {
        matches!(self, Token::Number(_))
    }

    pub fn match_trig(item: &str, num: f64) -> Result<Token, String> {
        let token = match item {
            "sin" => Token::Sin(num),
            "cos" => Token::Cos(num),
            "tan" => Token::Tan(num),
            "cot" => Token::Cot(num),
            "cosec" => Token::Cosec(num),
            "sec" => Token::Sec(num),
            _ => return Err("Invalid Named function".to_string()),
        };
        Ok(token)
    }

    pub fn match_symbol(ch: char) -> Result<Token, String> {
        let token = match ch {
            '^' => Token::Pow,
            '/' => Token::Div,
            '*' => Token::Mul,
            '+' => Token::Plus,
            '-' => Token::Minus,
            '(' => Token::LParan,
            ')' => Token::RParan,
            _ => return Err(format!("Invalid token {ch}")),
        };

        Ok(token)
    }
    pub fn precedence(&self) -> i16 {
        match self {
            Token::Pow => 5,
            Token::Div | Token::Mul => 3,
            Token::Plus | Token::Minus => 1,
            _ => 0,
        }
    }
}
