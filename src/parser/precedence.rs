use std::collections::{ HashMap, HashSet };

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Assoc { Left, Right, None }

#[derive(Debug, Clone)]
pub struct InfixLevel {
    pub symbols: HashSet<String>,
    pub assoc: Assoc
}

// TODO: different associativity can occur on the same precedence level
pub struct OperatorTable {
    // Higher = more closely binds to the operator (e.g., * higher than +)
    levels: [Option<InfixLevel>; 10],
    // <infix operator symbol> -> <level of precedence>
    lookup: HashMap<String, u32>
}

impl OperatorTable {
    pub fn new() -> OperatorTable {
        OperatorTable {
            levels: Default::default(),
            lookup: HashMap::new()
        }
    }

    pub fn add(&mut self, symbol: &str, precedence: u32, assoc: Assoc) {
        // let def = self.

        // self.lookup.insert(symbol.to_string(), def.clone());
        // self.levels[precedence as usize].push(def);
    }
}
