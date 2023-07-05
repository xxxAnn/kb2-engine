use std::collections::HashMap;

use crate::{utils::{parser::parse_item_list}, defs::{ErrorType, special_item, Kb2Result}};

use super::{gamedata::{Item, GameData, Recipe}, Dump};

#[derive(Debug, Clone)]
pub struct Inventory {
    pairs: HashMap<usize, u64>
}

impl Inventory {
    pub fn new(inv_inp: impl Into<String>) -> Self {
        let mut pairs = HashMap::new();

        let inv_str: String = inv_inp.into();

        if let Some(item_list) = parse_item_list(inv_str) {

            for (id, quantity) in item_list {
                pairs.insert(id, quantity);
            }

        }
        
        Self { pairs }.init()
    }

    fn init(mut self) -> Self {
        for item in special_item::LIST {
            self.add_item(*item, 0);
        }
        self
    }

    pub fn possible_recipes(&self, data: &GameData) -> Vec<(usize, Recipe)> {
        let recipes = data.get_recipes();
        recipes.iter().enumerate().map(|(id, r)| (id, r.clone())).filter(|(_, r)| {
            self.can_use_recipe(r)
        }).collect()
    }

    pub fn how_many_recipe(&self, r: &Recipe) -> u64 {
        r.inps().iter().map(|(i, q)| self.item_quantity(*i)/q).min().unwrap_or(0)
    }

    pub fn can_use_recipe(&self, r: &Recipe) -> bool {
        let mut t = 0;
        
        for (id, q) in r.inps() {
            t += i32::from(self.item_quantity(*id) < *q);
        }

        t == 0
    }

    pub fn item_quantity(&self, id: usize) -> u64 {
        let default = special_item::get_default(id);
        *self.pairs.get(&id).unwrap_or(&default)
    }

    pub fn get_total_exploit_multiplier(&self, data: &GameData) -> f32 {
        self.get_all_items(data).into_iter().map(|(i, q)| {
            i.exploit() * (q as f32)
        }).sum::<f32>().max(1.)
    }

    fn __craft(&mut self, rcp: &Recipe) {
        for (k, v) in rcp.inps() {
            self.remove_item(*k, *v)
        }
        for (k, v) in rcp.outs() {
            self.add_item(*k,*v)
        }
    } 

    pub fn craft(&mut self, rcp: &Recipe) -> Kb2Result<()> {
        if self.can_use_recipe(rcp) {
            self.__craft(rcp);
            Ok(())
        } else {
            Err(ErrorType::from("Recipe can't be crafted".to_owned()))
        }   
    }

    pub fn get_all_items(&self, data: &GameData) -> Vec<(Item, u64)> {
        let mut res = Vec::new();

        for (k, v) in &self.pairs {
            if let Some(item) = data.get_item_by_id(*k) {
                res.push((item.clone(), *v));
            }
        }

        res
    }

    pub fn position(&self) -> (u64, u64) {
        (self.item_quantity(special_item::X_LOCATION), self.item_quantity(special_item::Y_LOCATION))
    }

    pub fn energy(&self) -> u64 {
        self.item_quantity(special_item::ENERGY)
    }

    fn raw_dump(&self, predicate: impl Fn(usize) -> bool) -> String {
        let mut res = String::new();
        for (k, v) in &self.pairs {
            if predicate(*k) { // !special_item::LIST.contains(k) {
                res = format!("{res},{k}:{v}");
            }
        }

        if res.len() > 1 {
            res[1..].to_string()
        } else {
            res
        }
    }

    pub fn dump_regular(&self) -> String {
        self.raw_dump(|k| !special_item::LIST.contains(&k))
    }
    
    #[allow(dead_code)]
    pub fn balance(&self) -> u64 {
        self.item_quantity(special_item::MONEY)
    }

    pub fn add_item(&mut self, id: usize, quantity: u64) {
        let cq = self.item_quantity(id);
        if id == special_item::ENERGY {
            self.pairs.insert(id, (cq+quantity).min(5));
        } else {
            self.pairs.insert(id, cq + quantity);
        }
    }

    #[allow(dead_code)]
    pub fn remove_item(&mut self, id: usize, quantity: u64) {
        let cq = self.item_quantity(id);
        self.pairs.insert(id, cq - quantity);
    }

    #[allow(dead_code)]
    pub fn clear(&mut self) {
        self.pairs = HashMap::new();
        self.pairs.insert(special_item::MONEY, 100);
    }
}

impl Dump for Inventory {
    fn dump(&self) -> String {
        self.raw_dump(|_| true)
    }
}



