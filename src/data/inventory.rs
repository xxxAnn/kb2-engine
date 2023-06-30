use std::collections::HashMap;

use crate::parser::parse_item_list;

use super::{gamedata::Item, Data};

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

    pub fn get_total_exploit_multiplier(&self, data: &Data) -> f32 {
        self.get_all_items(data).into_iter().map(|(i, q)| {
            i.exploit() * (q as f32)
        }).sum::<f32>().max(1.)
    }

    pub fn get_all_items(&self, data: &Data) -> Vec<(Item, u64)> {
        let mut res = Vec::new();

        for (k, v) in self.pairs.iter() {
            if let Some(item) = data.gamedata().get_item_by_id(*k) {
                res.push((item.clone(), *v))
            }
        }

        res
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

    pub fn remove_item(&mut self, id: usize, quantity: u64) {
        self.pairs.insert(id, self.pairs.get(&id).unwrap_or(&0) - quantity);
    }

    pub fn clear(&mut self) {
        self.pairs = HashMap::new();
        self.pairs.insert(0, 100);
    }
}




