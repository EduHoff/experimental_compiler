use std::{env, fs::read_to_string};

use experimental_compiler::compiler::lexer;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Error: Invalid usage!");
        eprintln!("Usage: {} <file.exl>", &args[0]);
        std::process::exit(1);
    }

    let file_path = &args[1];

    let contents = match read_to_string(file_path) {
        Ok(txt) => txt,
        Err(err) => {
            eprintln!("Error reading file '{}': {}", file_path, err);
            std::process::exit(1);
        }
    };

    println!("{}", &contents);

    let token_list = lexer::tokenize(&contents);

    for tk in token_list.iter() {
        println!("{:?}\n", tk);
    }

    std::process::exit(0);
}
