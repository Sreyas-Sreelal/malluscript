//mod executor;
mod ast;
mod lexer;
mod parser;
mod executor;

use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args[1].to_ascii_lowercase() == "run" {
        //println!("{}", args[2]);
        if args[2].ends_with(".guh") {
            let mut file = File::open(&args[2]).expect("File not found!!"); 
            let mut contents = String::new();
            file.read_to_string(&mut contents).expect("Something went wrong in reading contents of file");
            println!("{}",contents);
            let mut exec = executor::Executor::new();
            let tokens = lexer::Lexer::new(&contents);
            //println!("{:?}",tokens);
            let parsed = parser::SourceUnitParser::new().parse(tokens).unwrap();
            //println!("{:?}",parsed);
            exec.execute(parsed);
        } else {
            println!(
                "Invalid file format {:?} expected .guh",
                args[2].split('.').next()
            );
        }
    } else {
        println!("Error!!! : No way dude");
    }
}
