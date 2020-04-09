mod executor;
mod lexer;
mod parser;

pub fn run_file(source:&str) {
    let mut exec = executor::Executor::new();
    let tokens = lexer::Lexer::new(&source);
    let parsed = parser::parse(&source, tokens);
    if let Err(message) = exec.execute(&parsed) {
        println!("\n**[Execution Failed]**");
        println!(
            "{}\n^^^^{}",
            &source[(message.0).0..(message.0).1 + 1].trim(),
            message.1
        );
    }
}
