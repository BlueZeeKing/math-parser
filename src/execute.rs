use crate::parser::TreeNode;

impl Into<f64> for TreeNode {
    fn into(self) -> f64 {
        match self {
            TreeNode::Add(node, node2) => Into::<f64>::into(*node) + Into::<f64>::into(*node2),
            TreeNode::Subtract(node, node2) => Into::<f64>::into(*node) - Into::<f64>::into(*node2),
            TreeNode::Multiply(node, node2) => Into::<f64>::into(*node) * Into::<f64>::into(*node2),
            TreeNode::Divide(node, node2) => Into::<f64>::into(*node) / Into::<f64>::into(*node2),
            TreeNode::Exponent(node, node2) => Into::<f64>::into(*node).powf((*node2).into()),
            TreeNode::Literal(num) => num,
        }
    }
}
