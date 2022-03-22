mod executor;
mod lexer;
mod parser;

use rustyline::error::ReadlineError;
use rustyline::Editor;
use std::collections::HashMap;

pub fn run_file(source: &str) {
    let mut tokens = lexer::Lexer::new(source, HashMap::new(), 0);

    match parser::parse(source, &mut tokens) {
        Ok(parsed) => {
            let mut exec = executor::Executor::new(tokens.literal_table, tokens.symbol_lookup);
            if let Err(message) = exec.execute(&parsed) {
                println!("\n**[Execution Failed]**");
                if let Some(region) = source.get((message.0).0..=(message.0).1) {
                    println!("{}", region);
                }
                println!("^^^^{}", message.1);
            }
        }
        Err(message) => {
            println!("{}", message);
            std::process::exit(1);
        }
    }
}

pub fn run_interactive_shell() {
    println!(
        "
    +---------------------------------------------------------------+
    |                   Mallu Script                                |
    | Repository: https://www.github.com/sreyas-sreelal/malluscript |
    +---------------------------------------------------------------+
                        Version {}
    ",
        env!("CARGO_PKG_VERSION")
    );
    let mut rl = Editor::<()>::new();
    let mut exec = executor::Executor::new(HashMap::new(), HashMap::new());
    let mut perisit_table = HashMap::new();
    let mut offest = 0;
    loop {
        match rl.readline(">> ") {
            Ok(code) => {
                if code.trim().is_empty() {
                    continue;
                }
                rl.add_history_entry(code.as_str());

                let mut tokens = lexer::Lexer::new(&code, perisit_table.clone(), offest);
                match parser::parse(&code, &mut tokens) {
                    Ok(parsed) => {
                        exec.update_literal_table(tokens.literal_table);
                        perisit_table = tokens.symbol_lookup.clone();
                        offest = tokens.lookup_count;
                        exec.update_lookup_table(tokens.symbol_lookup);
                        if let Err(message) = exec.execute(&parsed) {
                            if let Some(region) = code.get((message.0).0..=(message.0).1) {
                                println!("{}", region);
                            }
                            println!("^^^^{}", message.1);
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
