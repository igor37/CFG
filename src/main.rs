
mod cfg;
use cfg::{Symbol, Order, ContextFreeGrammar, CfgRule, SymbolChain}; 
use cfg::{ascending_order, random_order};

macro_rules! from_expr {
    ( $x:expr, $y:expr ) => {
        CfgRule::from_expression($x.to_string(), $y.to_string(), false).unwrap()
    }
}
macro_rules! from_start_expr {
    ( $x:expr, $y:expr ) => {
        CfgRule::from_expression($x.to_string(), $y.to_string(), true).unwrap()
    }
}

fn main() {

    // let start = Symbol { terminal: false, start: true, label: "S".to_string() };
    // let nont0 = Symbol { terminal: false, start: false, label: "A".to_string() };
    // let a = Symbol { terminal: true, start: false, label: "a".to_string() };
    // let b = Symbol { terminal: true, start: false, label: "b".to_string() };
    //
    // let nont0_chain = SymbolChain::new(vec![nont0.clone()]);
    // let chain0 = SymbolChain::new(vec![a.clone()]);
    // let chain1 = SymbolChain::new(vec![b.clone()]);
    // let chain2 = SymbolChain::new(vec![a, nont0.clone()]);
    // let chain3 = SymbolChain::new(vec![nont0.clone(), b]);
    //
    // let rule0 = CfgRule::new(start.clone(), vec![nont0_chain]);
    // let rule1 = CfgRule::new(nont0.clone(), vec![chain0, chain1, chain2, chain3]);

    // let rule0 = CfgRule::from_expression("S->!A".to_string(), ",".to_string()).unwrap();
    // let rule1 = CfgRule::from_expression("A->a|b|a,!A|!A,b").unwrap();



    // let rule0 = from_start_expr!("S->!A", ",").unwrap();
    // let rule1 = from_expr!("A->a|b|a,!A|!A,b", ",").unwrap();

    let rule0 = from_start_expr!("S->!article,!subject,!verb", ",");
    let rule1 = from_expr!("article->Der ", ",");
    let rule2 = from_expr!("subject->Hund |Oktopus |Fisch ", ",");
    let rule3 = from_expr!("verb->schwimmt durch den Teich|sonnt sich in der Wiese", ",");

    let mut cfg = ContextFreeGrammar::new();
    cfg.add_rule(rule0);
    cfg.add_rule(rule1);
    cfg.add_rule(rule2);
    cfg.add_rule(rule3);

    let strings = cfg.generate_strings(ascending_order(), 6, 15);
    // let strings = cfg.generate_strings(random_order(), 30, 15);
    for s in strings {
        println!("{}", s);
    }
}


