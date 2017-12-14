
#[macro_use]
mod cfg;
use cfg::{ContextFreeGrammar, CfgRule}; 
use cfg::{ascending_order, random_order};

fn main() {

    let rule0 = from_start_expr!("S->!article,!subject,!verb", ",");
    let rule1 = from_expr!("article->Der ", ",");
    let rule2 = from_expr!("subject->Hund |Oktopus |Fisch ", ",");
    let rule3 = from_expr!("verb->schwimmt durch den Teich|sonnt sich in der Wiese", ",");

    let mut cfg = ContextFreeGrammar::new();
    cfg.add_rule(rule0);
    cfg.add_rule(rule1);
    cfg.add_rule(rule2);
    cfg.add_rule(rule3);

    // let strings = cfg.generate_strings(ascending_order(), 30, 15);
    let strings = cfg.generate_strings(random_order(), 6, 15);
    // let strings = cfg.generate_strings(random_order(), 30, 15);
    for s in strings {
        println!("{}", s);
    }
}


