//mod executor;
mod ast;
mod lexer;
mod parser;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args[1].to_ascii_lowercase() == "run" {
        //println!("{}", args[2]);
        if args[2].ends_with(".guh") {
            if let Ok(file) = File::open(&args[2]) {
                let lines = io::BufReader::new(file).lines();
                //let mut lineno = 1;
                for line in lines {
                    if let Ok(l) = line {
                        let lex = lexer::Lexer::new(&l);
                        println!("{:?}", parser::SourceUnitParser::new().parse(lex));
                        // lineno += 1;
                    }
                }
            }
        //let mut exec = executor::Executor::new(tokens);
        //exec.execute();
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
