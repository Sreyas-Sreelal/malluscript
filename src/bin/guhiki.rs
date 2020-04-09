extern crate guhiki;

use clap::{App, Arg};
use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let matches = App::new("guhiki")
        .version("0.0.1")
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg(Arg::with_name("FILE").multiple(false))
        .get_matches();

    if let Some(filename) = matches.value_of("FILE") {
        let mut file = File::open(filename).expect("File not found!!");
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .expect("Something went wrong in reading contents of file");
        guhiki::run_file(&contents);
    } else {
        guhiki::run_interactive_shell();
    }
}
