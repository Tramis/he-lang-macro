use crate::list::List;
use crate::types::*;
use std::collections::HashMap;

type Stack<T> = Vec<T>;


#[derive(Debug)]
pub struct Scope {
    macros: HashMap<String, List<Macro>>,
}

impl Scope {
    pub fn new() -> Scope {
        Scope {
            macros: HashMap::new(),
        }
    }

    pub fn push_macro(&mut self, m: Macro) {
        let entry = self.macros.entry(m.name.clone()).or_insert(List::new());

        entry.push(m);
    }

    pub fn match_macro(&self, m_name: &str, params_cnt: usize) -> Option<&Macro> {
        if let Some(macros) = self.macros.get(m_name) {
            macros.iter().filter(|x| x.from.len() == params_cnt).next()
        } else {
            None
        }
    }
}
