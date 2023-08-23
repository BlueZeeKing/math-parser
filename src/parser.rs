use crate::{
    intermediate::{IntermediateToken, IntermediateTokenStream},
    Error,
};

/// This is the final representation before the final execution. This is a tree of nodes
/// representing each operation or literal. This representation is organized based on order of
/// operations and parentheses.
#[derive(Debug, PartialEq, Clone)]
pub enum TreeNode {
    Add(Box<TreeNode>, Box<TreeNode>),
    Subtract(Box<TreeNode>, Box<TreeNode>),
    Multiply(Box<TreeNode>, Box<TreeNode>),
    Divide(Box<TreeNode>, Box<TreeNode>),
    Exponent(Box<TreeNode>, Box<TreeNode>),
    Literal(f64),
}

impl TryFrom<IntermediateTokenStream> for TreeNode {
    type Error = Error;

    fn try_from(value: IntermediateTokenStream) -> Result<Self, Self::Error> {
        parse_ir(&value.0)
    }
}

fn parse_ir(value: &[IntermediateToken]) -> Result<TreeNode, Error> {
    if value.len() > 0
        && (!matches!(value[0], IntermediateToken::Literal(_))
            && !matches!(value[0], IntermediateToken::Parentheses(_)))
    {
        return Err(Error::InvalidStartingNode(value[0].clone()));
    }

    if value.len() == 1 {
        return match &value[0] {
            IntermediateToken::Literal(num) => Ok(TreeNode::Literal(*num)),
            IntermediateToken::Parentheses(tokens) => parse_ir(&tokens),
            token => Err(Error::LoneOperator(token.to_owned())),
        };
    }

    for (index, token) in value.iter().enumerate().rev() {
        if token == &IntermediateToken::Add {
            return Ok(TreeNode::Add(
                Box::new(parse_ir(&value[..index])?),
                Box::new(parse_ir(&value[index + 1..])?),
            ));
        }

        if token == &IntermediateToken::Subtract {
            return Ok(TreeNode::Subtract(
                Box::new(parse_ir(&value[..index])?),
                Box::new(parse_ir(&value[index + 1..])?),
            ));
        }
    }

    for (index, token) in value.iter().enumerate().rev() {
        if token == &IntermediateToken::Multiply {
            return Ok(TreeNode::Multiply(
                Box::new(parse_ir(&value[..index])?),
                Box::new(parse_ir(&value[index + 1..])?),
            ));
        }

        if token == &IntermediateToken::Divide {
            return Ok(TreeNode::Divide(
                Box::new(parse_ir(&value[..index])?),
                Box::new(parse_ir(&value[index + 1..])?),
            ));
        }
    }

    for (index, token) in value.iter().enumerate().rev() {
        if token == &IntermediateToken::Exponent {
            return Ok(TreeNode::Exponent(
                Box::new(parse_ir(&value[..index])?),
                Box::new(parse_ir(&value[index + 1..])?),
            ));
        }
    }

    Err(Error::CouldNotValueToReturn(value.to_vec()))
}
