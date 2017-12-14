
extern crate rand;
use self::rand::Rng;

use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;

#[macro_export]
macro_rules! from_expr {
    ( $x:expr, $y:expr ) => {
        CfgRule::from_expression($x.to_string(), $y.to_string(), false).unwrap()
    }
}
#[macro_export]
macro_rules! from_start_expr {
    ( $x:expr, $y:expr ) => {
        CfgRule::from_expression($x.to_string(), $y.to_string(), true).unwrap()
    }
}

/// A `Symbol` is a terminal or nonterminal with an arbitrary length,
/// represented as String.
#[derive(Clone, Debug)]
pub struct Symbol {
    pub terminal: bool,
    pub start:    bool,
    pub label:    String,
}

impl Symbol {
    fn is_terminal(&self) -> bool {
        self.terminal
    }
}

#[derive(Copy, Clone)]
pub enum Order {
    ASCENDING{ count: usize },
    RANDOM,
}

pub fn ascending_order() -> Order { Order::ASCENDING { count: 0 } }
pub fn random_order() -> Order { Order::RANDOM }

// This way of using a counting variable for varying the output in ascending
// order is not ideal or efficient(produces MANY duplicates) but it works.
fn init_idx(order: Order, n: usize) -> usize {
    match order {
        Order::ASCENDING{count: c}  => return c % n,
        _                           => return rand::thread_rng().gen_range(0, n),
    }
}

fn new_order(order: Order, n: usize) -> Order {
    match order {
        Order::ASCENDING{count: c} => {
            let new = c / n;
            return Order::ASCENDING { count: new };
        },
        _ => return order,
    }
}

// for stuff loaded from file
struct CfgData {
    pub order: Order,
    pub num: u32,
    pub max_depth: u32,
    pub separator: String,
    pub rules: Vec<String>,
}

impl CfgData {
    fn new() -> Self {
        CfgData {
            order: ascending_order(),
            num: 0,
            max_depth: 10,
            separator: ";".to_string(),
            rules: Vec::new()
        }
    }
    // checks if all necessary values were set and at least one rule was added
    fn are_complete(&self) -> bool {
        if self.num == 0 || self.rules.len() == 0 { return false; }
        true
    }
}

/// A context-free grammar containing a set of rules. Can generate and return a
/// set of words belonging to this grammar.
#[derive(Clone)]
pub struct ContextFreeGrammar {
    rules: Vec<CfgRule>,
    non_terminals: Vec<Symbol>,
    start: Option<Symbol>,
    order: Order,
    num:   u32,
    max_depth: u32,
}

impl ContextFreeGrammar {
    pub fn new() -> Self {
        ContextFreeGrammar {
            rules:   Vec::new(),
            non_terminals: Vec::new(),
            start: None,
            order: ascending_order(),
            num: 0,
            max_depth: 10,
        }
    }

    pub fn from_file(path: &str) -> Option<Self> {
        // open file and create buffered reader
        let f = match File::open(&Path::new(path)) {
            Err(e) => {
                println!("CFG::from_file(): {}", e);
                return None;
            },
            Ok(f)  => f,
        };
        let mut reader = BufReader::new(f);
        let mut line = String::new();
        let mut len;

        let mut data = CfgData::new();

        // read metadata
        loop {
            match reader.read_line(&mut line) {
                Err(e)  => {
                    println!("CFG::from_file(): {}", e);
                    return None;
                },
                Ok(l)   => len = l,
            }
            if len == 0 { break; }

            ContextFreeGrammar::parse_line(&line, &mut data);

            line.clear();
        }

        if !data.are_complete() { return None; }

        // parse & add rules
        let sep = data.separator;
        let mut rules: Vec<CfgRule> = data.rules.into_iter()
            .map(|r| CfgRule::from_expression(r, sep.clone(), false))
            .filter(|opt| opt.is_some())
            .map(|opt| opt.unwrap())
            .collect();
        rules[0].make_starting_rule();

        let mut cfg = ContextFreeGrammar::new();
        for i in 0..rules.len() {
            cfg.add_rule(rules[i].clone());
        }
        // other parameters
        cfg.set_order(data.order);
        cfg.set_num(data.num);
        cfg.set_max_depth(data.max_depth);

        Some(cfg)
    }

    fn parse_line(l: &String, data: &mut CfgData) {
        let order_str = "order";
        let asc_str   = "ascending";
        let rnd_str   = "random";
        let sep_str   = "separator";
        let num_str   = "words";
        let depth_str = "maxdepth";

        let line = l.replace("\n", "");

        if line.trim().starts_with("#") { return; } // comment
        if line.trim().len() < 2 { return; } // empty line
        if line.starts_with(order_str) {
            let parts: Vec<String> = line.split("=")
                .map(|p| p.trim())
                .map(|p| p.to_string())
                .collect();
            if parts.len() < 2 { println!("no argument in line '{}'", line); return; }
            if parts[1].contains(asc_str) {
                data.order = ascending_order();
                return;
            } else if parts[1].contains(rnd_str) {
                data.order = random_order();
                return;
            } else {
                println!("Ignoring line '{}': no valid order detected", line);
                return;
            }
        }
        if line.starts_with(sep_str) {
            let parts: Vec<String> = line.split("=")
                .map(|p| p.trim())
                .map(|p| p.to_string())
                .collect();
            if parts.len() < 2 { println!("no argument in line '{}'", line); return; }
            data.separator = parts[1].clone();
            return;
        }
        if line.starts_with(num_str) {
            let parts: Vec<String> = line.split("=")
                .map(|p| p.trim())
                .map(|p| p.to_string())
                .collect();
            if parts.len() < 2 { println!("no argument in line '{}'", line); return; }
            let num = match parts[1].parse::<u32>() {
                Err(_) => {
                    println!("Error in line '{}': no valid integer", line);
                    return;
                },
                Ok(n)  => n,
            };
            data.num = num;
            return;
        }
        if line.starts_with(depth_str) {
            let parts: Vec<String> = line.split("=")
                .map(|p| p.trim())
                .map(|p| p.to_string())
                .collect();
            if parts.len() < 2 { println!("no argument in line '{}'", line); return; }
            let dep = match parts[1].parse::<u32>() {
                Err(_) => {
                    println!("Error in line '{}': no valid integer", line);
                    return;
                },
                Ok(d)  => d,
            };
            data.max_depth = dep;
            return;
        }
        // else it's a normal rule
        data.rules.push(line.clone());
    }

    pub fn set_order(&mut self, order: Order) { self.order = order; }
    pub fn set_num(&mut self, n: u32) { self.num = n; }
    pub fn set_max_depth(&mut self, md: u32) { self.max_depth = md; }

    /// Adds a `CfgRule` to the grammar
    ///
    /// Returns false(and outputs an error message) if something went wrong.
    pub fn add_rule(&mut self, rule: CfgRule) -> bool {
        // check for duplicates; two non-terminals are not allowed to be defined
        // multiple times and only one starting non-terminal can exist
        let new_left = rule.get_left();

        for nt in 0..self.non_terminals.len() {
            // duplicate
            if self.non_terminals[nt].label == new_left.label {
                println!("Not adding rule: Duplicate detected.");
                return false;
            }
        }
        // if starting point
        if new_left.start { 
            // duplicate
            if self.start.is_some() {
                println!("Not adding rule: Duplicate starting point detected.");
                return false;
            }
            self.start = Some(new_left.clone());
        }

        // no duplicate was found
        self.non_terminals.push(new_left);

        self.rules.push(rule);
        true
    }

    /// Generates the given amount of words from this grammar in the given order
    /// and with the given max amount of non-terminal replacements(`max_depth`).
    ///
    /// If duplicates are not allowed, this function may return fewer results
    /// than requested.
    pub fn generate_strings(&self, allow_dupes: bool) -> Vec<String> {
        let mut order = self.order;
        let num = self.num;
        let max_depth = self.max_depth;

        if self.start.is_none() {
            println!("Generating not possible: no starting non-terminal set");
            return Vec::new();
        }
        if max_depth > 64 {
            println!("Warning: max_depth should be 64 at most for ascending order");
        }

        let mut results = Vec::new();

        // look for starting point
        for r in 0..self.rules.len() {
            let left = self.rules[r].get_left();

            if left.label == self.start.clone().unwrap().label {
                break;
            }
        }

        // generate `num` new strings
        let mut successive_dupes = 0;
        let mut n = 0;
        while n < num {
            let remaining_depth = max_depth;

            let result = self.generate(order, self.start.clone().unwrap(), remaining_depth);
            // check for duplicates
            if !allow_dupes {
                let redundant = results.contains(&result);
                if !redundant {
                    results.push(result);
                    n += 1;
                    successive_dupes = 0;
                } else {
                    successive_dupes += 1;
                    if successive_dupes >= num.max(20) { break; }
                }
            } else {
                n += 1;
            }

            order = match order {
                Order::ASCENDING{count: c} => Order::ASCENDING { count: c+1 },
                _ => order,
            };
        }

        results.clone()
    }

    fn generate(&self, mut order: Order, nonterm: Symbol, rem: u32) -> String {
        // return nothing if maximum depth was reached
        if rem == 0 { return "".to_string(); }

        let idx;

        // look for rule that fits to given non-terminal
        let mut outcome = None;
        for r in 0..self.rules.len() {
            if self.rules[r].get_left().label == nonterm.label {
                idx = init_idx(order, self.rules[r].num_of_outcomes());
                outcome = Some(self.rules[r].get_outcome(idx));
                order = new_order(order, self.rules[r].num_of_outcomes());
                break;
            }
        }
        if outcome.is_none() {
            panic!("Cannot generate: No rule for non-terminal {}", nonterm.label);
        }

        // build resulting string recursively
        let mut result = "".to_string();
        for symbol in outcome.unwrap().symbols {
            if symbol.is_terminal() {
                result = format!("{}{}", result, symbol.label);
            } else {
                result = format!("{}{}", result, self.generate(order, symbol.clone(), rem-1));
            }
        }

        result
    }

}

#[derive(Clone)]
pub struct CfgRule {
    left:  Symbol,
    outcomes: Vec<SymbolChain>,
}

impl CfgRule {
    /// Creates a new rule from a given expression as `String`.
    ///
    /// The syntax must be as follows:
    /// The left side(one non-terminal) is sepated from the right side with "->".
    /// The right side consists of one or more possible outcomes, separated by a
    /// "|" symbol. The (non-)terminals within the rules are separated with any
    /// other unused string not containing space(given as second parameter).
    /// Non-terminals should be preceded by a "!".
    ///
    /// Examples: "A->a|a,b,B" "the-> dog| cat| adjective, person"
    pub fn from_expression(expr: String, sym_separator: String, starting_rule: bool) -> Option<Self> {
        if sym_separator == "|".to_string() ||
                    sym_separator == "->".to_string() {
            println!("'|' and '->' are not allowed as separators");
            return None;
        }

        let sides: Vec<String> = expr.split("->")
                                        .map(|s| s.to_string())
                                        .collect();
        if sides.len() < 2 { println!("no right side in rule '{}'", expr); return None; }
        let left = sides[0].clone();
        let right = sides[1].clone();

        let mut outcomes = Vec::new();
        let raw_outcomes: Vec<String> = right.split("|")
                                                .map(|s| s.to_string())
                                                .collect();
        for i in 0..raw_outcomes.len() {
            if raw_outcomes[i].len() == 0 {
                println!("nothing on right side of rule '{}'", expr);
                return None;
            }

            let mut symbols = Vec::new();
            let ro = raw_outcomes[i].clone();
            let words: Vec<String> = ro.split(sym_separator.as_str())
                                        .map(|s| s.to_string())
                                        .collect();
            for n in 0..words.len() {
                let mut word = words[n].clone();
                if word.len() == 0 { return None; }
                let mut terminal = true;
                if word.starts_with("!") {
                    word = word.replace("!", "");
                    terminal = false;
                }

                let symbol = Symbol { terminal: terminal, start: false, label: word };
                symbols.push(symbol);
            }
            let chain = SymbolChain::new(symbols);

            outcomes.push(chain);
        }

        let left_symbol = Symbol {
            terminal: false,
            start: starting_rule,
            label: left
        };

        Some(CfgRule {
            left:       left_symbol,
            outcomes:   outcomes,
        })
    }

    fn make_starting_rule(&mut self) { self.left.start = true; }

    /// Returns the `SymbolChain` at the given index.
    ///
    /// The index cannot be out of bounds.
    fn get_outcome(&self, idx: usize) -> SymbolChain {
        let act_idx = idx % self.outcomes.len();

        self.outcomes[act_idx].clone()
    }

    pub fn num_of_outcomes(&self) -> usize {
        self.outcomes.len()
    }

    fn get_left(&self) -> Symbol { self.left.clone() }
}

/// A string of non-terminals and terminals
#[derive(Clone, Debug)]
pub struct SymbolChain {
    pub symbols: Vec<Symbol>,
}

impl SymbolChain {
    fn new(s: Vec<Symbol>) -> Self {
        SymbolChain {
            symbols: s,
        }
    }
}
