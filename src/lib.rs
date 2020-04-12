mod encoding;
mod executor;
mod lexer;
mod parser;

use crate::encoding::to_ascii;
use rustyline::error::ReadlineError;
use rustyline::Editor;

pub fn run_file(source: &str) {
    let mut exec = executor::Executor::new();
    let tokens = lexer::Lexer::new(&source);
    match parser::parse(&source, tokens) {
        Ok(parsed) => {
            if let Err(message) = exec.execute(&parsed) {
                println!("\n**[Execution Failed]**");
                println!(
                    "{}\n^^^^{}",
                    &source[(message.0).0..(message.0).1].trim(),
                    message.1
                );
            }
        }
        Err(message) => {
            println!("{}", message);
            std::process::exit(1);
        }
    }
}

pub fn run_interactive_shell() {
    println!("Mallu Script Version {}", env!("CARGO_PKG_VERSION"));
    println!("Repository: https://www.github.com/sreyas-sreelal/malluscript");
    let mut rl = Editor::<()>::new();
    let mut exec = executor::Executor::new();

    loop {
        match rl.readline(">> ") {
            Ok(code) => {
                if code.trim().is_empty() {
                    continue;
                }
                rl.add_history_entry(code.as_str());
                let code = to_ascii(code);

                match parser::parse(&code, lexer::Lexer::new(&code)) {
                    Ok(parsed) => {
                        if let Err(message) = exec.execute(&parsed) {
                            println!(
                                "{}\n^^^^{}",
                                &code[(message.0).0..(message.0).1].trim(),
                                message.1
                            );
                        }
                    }
                    Err(message) => {
                        println!("{}", message);
                    }
                }
            }
            Err(ReadlineError::Interrupted) | Err(ReadlineError::Eof) => {
                println!("Sed :( Bei Bei");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
}
