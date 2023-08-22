use math_parser::{
    intermediate::IntermediateTokenStream, lexer::LexTokenStream, parse, parser::TreeNode,
};

fn main() {
    println!("{}", parse("(3+1)*-8^2").unwrap());
}
