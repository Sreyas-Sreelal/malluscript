pub mod executor;
pub mod lexer;
pub mod parser;

#[cfg(not(target_arch = "wasm32"))]
use rustyline::error::ReadlineError;
#[cfg(not(target_arch = "wasm32"))]
use rustyline::Editor;
use std::collections::HashMap;

pub fn run_source(source: &str) -> Result<String, String> {
    let mut tokens = lexer::Lexer::new(source, HashMap::new(), 0);

    match parser::parse(source, &mut tokens) {
        Ok(parsed) => {
            let mut exec = executor::Executor::new(tokens.literal_table, tokens.symbol_lookup);
            match exec.execute(&parsed) {
                Ok(_) => Ok(exec.output.join("")),
                Err(message) => {
                    let mut err_str = String::from("\n**[Execution Failed]**\n");
                    let (start, end) = message.0;
                    let mut s = start;
                    while s > 0 && !source.is_char_boundary(s) {
                        s -= 1;
                    }
                    let mut e = end + 1;
                    while e <= source.len() && !source.is_char_boundary(e) {
                        e += 1;
                    }
                    if let Some(region) = source.get(s..e) {
                        err_str.push_str(&format!("{}\n", region));
                    }
                    err_str.push_str(&format!("^^^^{}\n", message.1));
                    Err(err_str)
                }
            }
        }
        Err(message) => Err(format!("{}", message)),
    }
}

pub fn store_result(source: &str) -> Vec<String> {
    match run_source(source) {
        Ok(output) => vec![output],
        Err(err) => vec![err],
    }
}

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

#[cfg(not(target_arch = "wasm32"))]
pub fn run_interactive_shell() {
    println!(
        "
    ╭───────────────────────────────────────────────────────────────╮
    │                       Mallu Script                            │
    │ Repository: https://www.github.com/sreyas-sreelal/malluscript │
    ╰───────────────────────────────────────────────────────────────╯
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
