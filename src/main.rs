mod executor;
mod lexer;
use lexer::tokenize;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args[1].to_ascii_lowercase() == "run" {
        //println!("{}", args[2]);
        if args[2].ends_with(".guh") {
            let tokens = tokenize(&args[2]);
            //println!("{:?}", tokens);
            let mut exec = executor::Executor::new(tokens);
            exec.execute();
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
