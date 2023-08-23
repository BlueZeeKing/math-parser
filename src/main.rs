use std::io::stdin;

use math_parser::{
    intermediate::IntermediateTokenStream, lexer::LexTokenStream, parser::TreeNode, Error,
};

fn main() -> Result<(), Error> {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();

    let lex_stream = dbg!(input.parse::<LexTokenStream>()?);

    let intermediate_stream: IntermediateTokenStream = dbg!(lex_stream.try_into()?);

    let tree: TreeNode = dbg!(intermediate_stream.try_into()?);

    let result: f64 = tree.into();

    println!("Answer: {result}");

    Ok(())
}
