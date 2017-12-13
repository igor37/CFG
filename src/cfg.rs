
extern crate rand;
use self::rand::Rng;

#[derive(Clone, Debug)]
pub struct Symbol {
    pub terminal: bool,
    pub start:    bool,
    pub label:    String,
}

impl Symbol {
    fn new_empty() -> Self {
        Symbol {
            terminal: true,
            start:    false,
            label:    "".to_string(),
        }
    }

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

fn init_idx(order: Order, n: usize) -> usize {
    match order {
        Order::ASCENDING{count: c}  => return c % n,
        _                           => return rand::thread_rng().gen_range(0, n),
    }
}

fn new_order(order: Order, n: usize) -> Order {
    match order {
        // Order::ASCENDING{c} => return (old_idx + 1, Order::ASCENDING{ count: c+1 }),
        Order::ASCENDING{count: c} => {
            // let mut new = c / (n+1);
            let mut new = c / n;
            return Order::ASCENDING { count: new };
        },
        // Order::RANDOM       => return (rand::thread_rng().gen_range(0, usize::max_value()), order),
        _ => return order,
    }
}

#[derive(Clone)]
pub struct ContextFreeGrammar {
    rules: Vec<CfgRule>,
    non_terminals: Vec<Symbol>,
    start: Option<Symbol>,
}

impl ContextFreeGrammar {
    pub fn new() -> Self {
        ContextFreeGrammar {
            rules:   Vec::new(),
            non_terminals: Vec::new(),
            start: None,
        }
    }

    pub fn add_rule(&mut self, rule: CfgRule) -> bool {
        // check for duplicates; two non-terminals are not allowed to be defined
        // multiple times and only one starting non-terminal can exist
        let mut new_left = rule.get_left();

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

    // pub fn is_ready(&self, verbose: bool) -> bool {
    //     if self.start.is_none() {
    //         if verbose { println!("Starting rule is missing"); }
    //         return false;
    //     }
    //     // TODO check if rule exists for every nonterminal
            //
            // true
    // }

    // fn get_outcomes_for(&self, symbol: Symbol) -> Vec<SymbolChain> {
    //     for r in self.rules {
    //         if r.get_left().label == symbol.label {
    //             return r.get_outcomes();
    //         }
    //     }
    //
    //     Vec::new()
    // }

    pub fn generate_strings(&self, mut order: Order, num: u32, max_depth: u32) -> Vec<String> {
        if self.start.is_none() {
            println!("Generating not possible: no starting non-terminal set");
            return Vec::new();
        }
        if max_depth > 64 {
            println!("Warning: max_depth should be 64 at most for ascending order");
        }

        let mut results = Vec::new();
        // let mut idx = next_idx(order, 0);

        // look for starting point
        let mut starting_idx = 0;
        for r in 0..self.rules.len() {
            let left = self.rules[r].get_left();

            if left.label == self.start.clone().unwrap().label {
                starting_idx = r;
                break;
            }
        }

        // generate `num` new strings
        let mut n = 0;
        while n < num {
            let mut remaining_depth = max_depth;

            let result = self.generate(order, self.start.clone().unwrap(), remaining_depth);
            let redundant = results.contains(&result);
            if !redundant {
                results.push(result);
                n += 1;
            }

            order = match order {
                Order::ASCENDING{count: c} => Order::ASCENDING { count: c+1 },
                _                   => order,
            };
        }

        results.clone()
    }

    fn generate(&self, mut order: Order, nonterm: Symbol, rem: u32) -> String {
        // return nothing if maximum depth was reached
        if rem == 0 { return "".to_string(); }

        let mut idx = 0;

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
    pub fn new(left: Symbol, chains: Vec<SymbolChain>) -> Self {
        if chains.len() == 0 { panic!("At least 1 possible outcome must exist"); }
        CfgRule {
            left: left,
            outcomes: chains,
        }
    }

    /// Creates a new rule from a given expression as `String`.
    ///
    /// The syntax must be as follows:
    /// The left side(one non-terminal) is sepated from the right side with "->".
    /// The right side consists of one or more possible outcomes, separated by a
    /// "|" symbol. The (non-)terminals within the rules are separated with any
    /// other unused character(given as second parameter). Non-terminals should
    /// be preceded by a "!".
    ///
    /// Examples: "A->a|a,b,B" "the-> /dog| /cat| /person"
    pub fn from_expression(expr: String, sym_separator: String, starting_rule: bool) -> Option<Self> {
        if sym_separator == "|".to_string() { return None; }

        let sides: Vec<String> = expr.split("->").map(|s| s.to_string()).collect();
        let left = sides[0].clone();
        let right = sides[1].clone();

        let mut outcomes = Vec::new();
        let raw_outcomes: Vec<String> = right.split("|").map(|s| s.to_string()).collect();
        for i in 0..raw_outcomes.len() {
            if raw_outcomes[i].len() == 0 { return None; }

            let mut symbols = Vec::new();
            // let ro: &str = raw_outcomes[i].as_str();
            let ro = raw_outcomes[i].clone();
            let words: Vec<String> = ro.split(sym_separator.as_str()).map(|s| s.to_string()).collect();
            // let words = &raw_outcomes[i].split(sym_separator).collect();
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

        let left_symbol = Symbol { terminal: false, start: starting_rule, label: left };

        // println!("{:?} -> {:?}", left_symbol, outcomes);

        Some(CfgRule {
            left:       left_symbol,
            outcomes:   outcomes,
        })
    }

    /// Returns the `SymbolChain` at the given index
    ///
    /// If the index is out of bounds, loop around
    fn get_outcome(&self, idx: usize) -> SymbolChain {
        let act_idx = idx % self.outcomes.len();

        self.outcomes[act_idx].clone()
    }

    pub fn num_of_outcomes(&self) -> usize {
        self.outcomes.len()
    }

    fn get_left(&self) -> Symbol { self.left.clone() }

    fn get_outcomes(&self) -> Vec<SymbolChain> {
        self.outcomes.clone()
    }
}

#[derive(Clone, Debug)]
pub struct SymbolChain {
    pub symbols: Vec<Symbol>,
}

impl SymbolChain {
    pub fn new(s: Vec<Symbol>) -> Self {
        SymbolChain {
            symbols: s,
        }
    }

    pub fn add_symbol(&mut self, c: Symbol) {
        self.symbols.push(c);
    }
}
