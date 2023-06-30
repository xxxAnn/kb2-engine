use std::collections::HashMap;

use crate::parser::parse_item_list;

#[derive(Debug, Clone)]
pub struct Inventory {
    pairs: HashMap<usize, u64>
}

impl Inventory {
    pub fn new(inv_inp: impl Into<String>) -> Self {
        let mut pairs = HashMap::new();

        let inv_str: String = inv_inp.into();

        for (id, quantity) in parse_item_list(inv_str) {
            pairs.insert(id, quantity);
        }

        Self { pairs }
    }

    pub fn dump(&self) -> String {
        let mut res = String::new();
        for (k, v) in self.pairs.iter() {
            res = format!("{},{}", res, format!("{}:{}", k, v));
        }

        res[1..].to_string()
    }

    pub fn balance(&self) -> u64 {
        *self.pairs.get(&0usize).unwrap_or(&0)
    }

    pub fn add_item(&mut self, id: usize, quantity: u64) {
        self.pairs.insert(id, self.pairs.get(&id).unwrap_or(&0) + quantity);
    }
}




