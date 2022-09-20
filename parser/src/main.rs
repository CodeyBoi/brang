use std::io::Read;

use token::tokenize;

mod compile;
mod token;
mod interpret;

fn main() {

    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        eprintln!("please input file");
    } else {
        match args[1].as_str() {
            "compile" => {
                let mut program = String::with_capacity(8192);
                let mut file = std::fs::File::open(&args[2]).unwrap();
                file.read_to_string(&mut program).unwrap();
                let tokens = token::tokenize(program);
                let root = token::parse(tokens);
                let output = if args.len() >= 4 && args[3] == "-o" {
                    &args[4]
                } else { "b.bf" };
                println!("{}", root);
                compile::to_brainfuck(root, output).expect("could not write to file");
                println!("Wrote program to '{}'.", output);
            }
            "run" => interpret::run(&args[2]),
            _ => eprintln!("unrecognized command: '{}'", args[1]),
        }
    }
}