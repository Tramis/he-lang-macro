use crate::types::*;
use std::collections::HashMap;
use std::iter::repeat;

#[derive(Debug)]
pub struct Scope {
    pub macros: HashMap<(String, usize), Macro>,
    pub nested_cnt: usize,
    pub stack: Vec<String>,
}

impl Scope {
    pub fn new() -> Scope {
        Scope {
            macros: HashMap::new(),
            nested_cnt: 0,
            stack: vec![],
        }
    }

    pub fn push_macro(&mut self, m: Macro) {
        self.macros.insert((m.name.clone(), m.from.len()), m);
    }

    pub fn match_macro(&self, m_name: &str, params_cnt: usize) -> Option<&Macro> {
        self.macros.get(&(m_name.to_string(), params_cnt))
    }

    pub fn push_cnt(&mut self) {
        self.nested_cnt += 1;
    }

    pub fn pop_cnt(&mut self) {
        self.nested_cnt -= 1;
    }

    pub fn make_tab(&self) -> String {
        repeat("    ").take(self.nested_cnt).collect()
    }
}
