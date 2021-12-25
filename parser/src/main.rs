use std::io::Read;

use token::tokenize;

mod parse;
mod token;
mod interpret;

fn main() {
    let mut program = String::with_capacity(8192);
    std::io::stdin().read_to_string(&mut program).unwrap();
    let tokens = tokenize(program);
    for token in &tokens {
        println!("{:?}", token);
    }

    let root = token::parse(tokens);
    token::print_tree(&root);

    parse::to_brainfuck(root, "tests/b.assem").unwrap();

    interpret::run("tests/b.assem");
}