use core::f64;

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
