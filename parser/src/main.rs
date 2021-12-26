use std::io::Read;

use token::tokenize;

mod parse;
mod token;
mod interpret;

fn main() {
    let mut program = String::with_capacity(8192);
    std::io::stdin().read_to_string(&mut program).unwrap();
    let tokens = tokenize(program);
    println!("tokens:\n{:#?}", tokens);

    let root = token::parse(tokens);
    println!("tree:\n{}", root);

    parse::to_brainfuck(root, "tests/b.bf").unwrap();

    interpret::run("tests/b.bf");
}