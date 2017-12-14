
#[macro_use]
mod cfg;
use cfg::{ContextFreeGrammar}; 

fn main() {

    let cfg = match ContextFreeGrammar::from_file("example_cfg") {
        None    => return,
        Some(c) => c,
    };

    let strings = cfg.generate_strings(false);

    for s in strings {
        println!("{}", s);
    }
}


