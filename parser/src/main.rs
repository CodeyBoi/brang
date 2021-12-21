use std::io::Read;

use token::tokenize;

mod compiler;
mod token;

fn main() {
    let mut program = String::with_capacity(8192);
    std::io::stdin().read_to_string(&mut program).unwrap();
    let tokens = tokenize(program);
    for token in &tokens {
        println!("{:?}", token);
    }

    let root = token::parse(tokens);
    token::print_tree(&root);

    compiler::to_bf(root, "../b.assem").unwrap();
}