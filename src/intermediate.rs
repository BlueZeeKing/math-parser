use crate::{
    lexer::{LexToken, LexTokenStream},
    Error,
};

#[derive(Debug, PartialEq, Clone)]
pub enum IntermediateToken {
    Add,
    Subtract,
    Multiply,
    Divide,
    Exponent,
    Parentheses(Vec<IntermediateToken>),
    Literal(f64),
}

/// This is one step above the [`LexTokenStream`]. It is still mostly flat except for parentheses.
/// This representation also disambiguates [`LexToken::Minus`] tokens into either negative signs
/// (-1 * ...) or true subtraction.
#[derive(Debug)]
pub struct IntermediateTokenStream(pub(crate) Vec<IntermediateToken>);

impl TryFrom<LexTokenStream> for IntermediateTokenStream {
    type Error = Error;

    fn try_from(mut value: LexTokenStream) -> Result<Self, Self::Error> {
        let mut index = 0;

        value.0.push(LexToken::ParensClose);

        let stream = IntermediateTokenStream(parse_stream_part(&value.0, &mut index)?);

        if index + 1 != value.0.len() {
            return Err(Error::ExtraParentheses);
        }

        Ok(stream)
    }
}

fn parse_stream_part(
    value: &[LexToken],
    index: &mut usize,
) -> Result<Vec<IntermediateToken>, Error> {
    let mut stream = Vec::new();

    while *index < value.len() {
        let token = &value[*index];

        match token {
            LexToken::Plus => stream.push(IntermediateToken::Add),
            LexToken::Star => stream.push(IntermediateToken::Multiply),
            LexToken::Slash => stream.push(IntermediateToken::Divide),
            LexToken::Exponent => stream.push(IntermediateToken::Exponent),
            LexToken::Number(num) => {
                stream.push(IntermediateToken::Literal(*num));

                if *index + 1 < value.len() && value[*index + 1] == LexToken::ParensOpen {
                    stream.push(IntermediateToken::Multiply);
                }
            }
            LexToken::Minus => {
                if *index == 0
                    || [
                        LexToken::Plus,
                        LexToken::Minus,
                        LexToken::Star,
                        LexToken::Slash,
                        LexToken::ParensOpen,
                    ]
                    .contains(&value[*index - 1])
                {
                    stream.reserve(2);
                    stream.push(IntermediateToken::Literal(-1.0));
                    stream.push(IntermediateToken::Multiply);
                } else {
                    stream.push(IntermediateToken::Subtract);
                }
            }
            LexToken::ParensOpen => {
                *index += 1;
                stream.push(IntermediateToken::Parentheses(parse_stream_part(
                    value, index,
                )?));
            }
            LexToken::ParensClose => {
                return Ok(stream);
            }
        }

        *index += 1;
    }

    Err(Error::ParenthesesNotClosed)
}
