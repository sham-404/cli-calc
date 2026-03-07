#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Number(f64),

    Deg,
    Rad,

    Plus,
    Minus,
    Mul,
    Div,
    Pow,

    LParan,
    RParan,

    Sin,
    Cos,
    Tan,
    Cot,
    Sec,
    Cosec,
}

impl Token {
    pub fn is_operator(&self) -> bool {
        matches!(
            self,
            Token::Pow
                | Token::Div
                | Token::Mul
                | Token::Plus
                | Token::Minus
                | Token::Sin
                | Token::Cos
                | Token::Tan
                | Token::Cot
                | Token::Sec
                | Token::Cosec
                | Token::Deg
                | Token::Rad
        )
    }

    pub fn is_trig(&self) -> bool {
        matches!(
            self,
            Token::Sin | Token::Cos | Token::Tan | Token::Cot | Token::Sec | Token::Cosec
        )
    }

    pub fn is_num(&self) -> bool {
        matches!(self, Token::Number(_))
    }

    pub fn match_trig(item: &str) -> Result<Token, String> {
        let token = match item {
            "sin" => Token::Sin,
            "cos" => Token::Cos,
            "tan" => Token::Tan,
            "cot" => Token::Cot,
            "cosec" => Token::Cosec,
            "sec" => Token::Sec,

            "degree" | "deg" => Token::Deg,
            "radian" | "rad" => Token::Rad,

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
            Token::Pow => 6,
            Token::Deg | Token::Rad => 5,
            Token::Sin | Token::Cos | Token::Tan | Token::Cot | Token::Cosec | Token::Sec => 4,
            Token::Div | Token::Mul => 3,
            Token::Plus | Token::Minus => 1,
            _ => 0,
        }
    }
}
