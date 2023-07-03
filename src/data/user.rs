use crate::{game::Summary, defs::{ErrorType, BASE_QUANTITY}};

use super::{db::DBConnection, inventory::Inventory, gamedata::{Item, GameData, Recipe}};
use rand::prelude::*;

pub struct User {
    id: u64,
    inventory: Inventory,
    connector: DBConnection
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
        User::new(self.id)
    }
}

impl User {
    pub fn new(id: u64) -> Self {
        let connector = DBConnection::new();
        let inventory = connector.get_player_inventory(id);

        Self {
            id,
            inventory,
            connector
        }
    }    

    #[allow(dead_code)]
    pub fn money(&self) -> u64 {
        self.inventory.balance()
    }

    pub fn id(&self) -> u64 {
        self.id
    }

    pub fn add_item(&mut self, item_id: usize, quantity: u64) {
        self.inventory.add_item(item_id, quantity);
        self.save();
    }

    #[allow(dead_code)]
    pub fn remove_item(&mut self, item_id: usize, quantity: u64) {
        self.inventory.remove_item(item_id, quantity);
        self.save();
    }

    pub fn save(&mut self) {
        let inv_str = self.inventory.dump();
        
        self.connector.update_player_inventory(self.id, inv_str);
    }

    pub fn max_can_craft(&self, r: &Recipe) -> u64 {
        self.inventory.how_many_recipe(r)
    }

    fn get_exploit_quantity(&self, gm: &GameData) -> u64 {
        let mut rng = rand::thread_rng();
        let pcng: f32 = rng.gen();

        ((pcng + 0.1)/(1.0) * self.get_total_multiplier(gm) as f32 * BASE_QUANTITY).ceil() as u64
    }

    fn select_exploit_item(&self, gamedata: &GameData) -> Option<Item> {
        let items = gamedata.get_exploitable();

        let max: u32 = items.iter().map(|(_, v)| v).sum();

        let mut rng = rand::thread_rng();
        let mut num = rng.gen_range(0..max) + 1;

        let mut temp = None;

        for (item, weight) in items {
            temp = Some(item.clone());

            if weight >= num {
                break;
            }

            num -= weight;
        }

        temp
    }

    pub fn exploit(&mut self, gamedata: &GameData) -> Vec<(Item, u64)> {
        let mut res = Vec::new();

        res.push(
            (
                self.select_exploit_item(gamedata).unwrap(), // this shouldn't error unless the object table is modified
                self.get_exploit_quantity(gamedata)
            )
        ); 

        for (el, q) in &res {
            self.add_item(el.id(), *q);
        }

        res
    }

    pub fn craft(&mut self, gamedata: &GameData, recipe_id: usize, quantity: u64) -> Result<Recipe, ErrorType> {
        let rcp = quantity * gamedata.get_recipe_by_id(recipe_id).ok_or("Recipe not found".to_owned())?;

        self.inventory.craft(&rcp)?;
        self.save();

        Ok(rcp.clone())
    } 

    #[allow(dead_code)]
    pub fn clear_inventory(&mut self) {
        self.inventory.clear();
        self.save();
    }

    fn get_total_multiplier(&self, gd: &GameData) -> f32 {
        self.inventory.get_total_exploit_multiplier(gd)
    }

    pub fn possible_recipes(&self, gd: &GameData) -> Vec<(usize, Recipe)> {
        self.inventory.possible_recipes(gd)
    }
}

impl ToString for User {
    fn to_string(&self) -> String {
        format!("{}={}", self.id, self.inventory.dump())
    }
}
impl Summary for User {
    fn text(&self) -> String {
        format!("{}\r\n{}", "get_user_", self.to_string())
    }
}