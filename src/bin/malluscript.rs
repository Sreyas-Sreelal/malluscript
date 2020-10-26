extern crate malluscript;

use clap::{App, Arg};
use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let matches = App::new("guhiki")
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg(Arg::with_name("FILE").multiple(false))
        .get_matches();

    if let Some(filename) = matches.value_of("FILE") {
        let mut file = File::open(filename).expect("File not found!!");
        if !filename.ends_with(".ms") {
            println!("[Error]: Malluscript programs should be written in file with .ms extension");
            return;
        }
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .expect("Something went wrong in reading contents of file");
        malluscript::run_file(&contents);
    } else {
        malluscript::run_interactive_shell();
    }
}