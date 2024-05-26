#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Assoc { Left, Right, None }

#[derive(Debug)]
pub struct InfixDef {
    pub symbol: String,

    // Higher = more closely binds to the operator (e.g., * higher than +)
    pub precedence: u32,
    pub assoc: Assoc
}

pub struct OperatorTable {
    levels: Vec<Vec<InfixDef>>,
    lookup: std::collections::HashMap<String, InfixDef>
}
