
// for reading command line arguments
use std::env;

#[macro_use]
mod cfg;
use cfg::{ContextFreeGrammar}; 

fn main() {
    let file;
    let mut spacing = false;

    let args: Vec<_> = env::args().collect();
    if args.len() > 1 {
        if args[1] == "-h".to_string() ||
           args[1] == "--help".to_string() {
            print_usage();
            return;
        }
        // file path
        file = args[1].clone();
        // spacing?
        if args.len() > 2 {
            if args[2] == "-s".to_string() {
                spacing = true;
            }
        }
    } else {
        print_usage();
        return;
    }

    // let cfg = match ContextFreeGrammar::from_file("example_cfg") {
    let cfg = match ContextFreeGrammar::from_file(file.as_str()) {
        None    => return,
        Some(c) => c,
    };

    let strings = cfg.generate_strings(false);

    for s in strings {
        println!("{}", s);
        if spacing { println!(""); }
    }
}

fn print_usage() {
    println!("Usage: cfg [-h|--help] OR cfg <file name> [-s]\n");
    println!("-s:           Empty line between results");
    println!("-h|--help:    Output this message");
}
