use std::str::FromStr;

use crate::Error;

#[derive(Debug, PartialEq, Clone)]
pub enum LexToken {
    Plus,
    Minus,
    Star,
    Slash,
    ParensOpen,
    ParensClose,
    Number(f64),
    Exponent,
}

#[derive(Debug)]
pub struct LexTokenStream(pub(crate) Vec<LexToken>);

impl FromStr for LexTokenStream {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut stream = Vec::new();
        let mut current_num: Option<String> = None;

        for char in s
            .chars()
            .filter(|char| !char.is_whitespace() && *char != ',')
        {
            if ['+', '-', '*', '/', '(', ')', '^'].contains(&char) && current_num.is_some() {
                stream.push(LexToken::Number(
                    current_num
                        .take()
                        .expect("Should never fail")
                        .parse::<f64>()?,
                ));
            }

            match char {
                '+' => stream.push(LexToken::Plus),
                '-' => stream.push(LexToken::Minus),
                '*' => stream.push(LexToken::Star),
                '/' => stream.push(LexToken::Slash),
                '^' => stream.push(LexToken::Exponent),
                '(' => stream.push(LexToken::ParensOpen),
                ')' => stream.push(LexToken::ParensClose),
                char => {
                    if char.is_numeric() || char == '.' {
                        current_num.get_or_insert_with(String::new).push(char);
                    } else {
                        return Err(Error::UnknownChar(char));
                    }
                }
            }
        }

        if let Some(num) = current_num.take() {
            stream.push(LexToken::Number(num.parse::<f64>()?));
        }

        Ok(LexTokenStream(stream))
    }
}
