use std::num::ParseFloatError;

use intermediate::{IntermediateToken, IntermediateTokenStream};
use lexer::LexTokenStream;
use parser::TreeNode;
use thiserror::Error;

pub mod execute;
pub mod intermediate;
pub mod lexer;
pub mod parser;

#[cfg(test)]
mod test;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Parentheses were not closed properly")]
    ParenthesesNotClosed,
    #[error("There were extra closing parentheses")]
    ExtraParentheses,
    #[error("{0} is not a valid char")]
    UnknownChar(char),
    #[error("The token stream {0:#?} could not be parsed properly")]
    CouldNotValueToReturn(Vec<IntermediateToken>),
    #[error("Could not parse string to number: {0}")]
    StringParse(#[from] ParseFloatError),
    #[error("Cannot parse lone operator {0:?}")]
    LoneOperator(IntermediateToken),
    #[error("The starting node {0:?} is invalid. (This is possible due to a double operator)")]
    InvalidStartingNode(IntermediateToken),
}

pub fn parse(input: &str) -> Result<f64, Error> {
    let lex_stream = input.parse::<LexTokenStream>()?;

    let intermediate_stream: IntermediateTokenStream = lex_stream.try_into()?;

    let tree: TreeNode = intermediate_stream.try_into()?;

    Ok(tree.into())
}
