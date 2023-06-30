use super::{db::DBConnection, Data, inventory::Inventory, gamedata::Item};
use rand::prelude::*;

pub struct User {
    id: u64,
    inventory: Inventory,
}

impl std::fmt::Debug for User {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("User")
            .field("id", &self.id)
            .field("inventory", &self.inventory)
            .finish()
    }
}

impl Clone for User {
    fn clone(&self) -> Self {
        Self { id: self.id.clone(), inventory: self.inventory.clone() }
    }
}

impl User {
    pub fn new(id: u64, connector: &DBConnection) -> Self {
        let inventory = connector.get_player_inventory(id);

        let money = inventory.balance();

        Self {
            id,
            inventory,
        }
    }    

    pub fn money(&self) -> u64 {
        self.inventory.balance()
    }

    pub fn id(&self) -> u64 {
        self.id
    }

    pub fn add_item(&mut self, item_id: usize, quantity: u64, data: &Data) {
        self.inventory.add_item(item_id, quantity);
        self.save(data);
    }

    pub fn remove_item(&mut self, item_id: usize, quantity: u64, data: &Data) {
        self.inventory.remove_item(item_id, quantity);
        self.save(data);
    }

    pub fn save(&mut self, data: &Data) {
        let inv_str = self.inventory.dump();
        
        data.update_player_inventory(self.id, inv_str);
    }

    pub fn exploit(&mut self, data: &Data) -> Vec<(Item, u64)> { // Vec<(Item obtained, Quantity)>
        let mut res = Vec::new();

        let items = data.gamedata().get_exploitable();

        let max: u32 = items.iter().map(|(_, v)| v).sum();

        let mut rng = rand::thread_rng();
        let mut num = rng.gen_range(0..max) + 1;

        let mut temp = data.gamedata().get_item_by_id(0).unwrap().clone();
        let mut temp_weight= 1;

        for (item, weight) in items {
            temp = item.clone();
            temp_weight = weight;

            if weight >= num {
                break;
            }

            num -= weight;
        }

        res.push((temp.clone(), temp_weight as u64 * self.get_total_multiplier(data)));

        for (el, q) in res.iter() {
            self.add_item(el.id(), *q, data)
        }

        res
    }

    pub fn clear_inventory(&mut self, data: &Data) {
        self.inventory.clear();
        self.save(data)
    }

    fn get_total_multiplier(&self, data: &Data) -> u64 {
        self.inventory.get_total_exploit_multiplier(data).floor() as u64
    }
}
