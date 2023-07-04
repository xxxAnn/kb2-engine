use std::time::{SystemTime, UNIX_EPOCH};

use crate::{game::Summary, defs::{ErrorType, BASE_QUANTITY, special_item, Kb2Result}};

use super::{db::DBConnection, inventory::Inventory, gamedata::{Item, GameData, Recipe}};
use rand::prelude::*;

pub struct User {
    id: u64,
    inventory: Inventory,
    last_energy_use: u64,
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
        let connector = DBConnection::new().expect("Could not connect to Database.");
        let inventory = connector.get_player_inventory(id);

        Self {
            id,
            inventory,
            last_energy_use: 0,
            connector
        }.init()
    }    

    fn init(mut self) -> Self {
        self.save();
        self
    }

    fn energy_use(&mut self) {
        let ctime = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap() // shouldn't panic unless the system clock is broken
            .as_secs();

        let time_delta = ctime - self.last_energy_use;

        if time_delta > 30 {
            self.last_energy_use = ctime;
            self.inventory.add_item(special_item::ENERGY, time_delta/30)
        }

        
    }

    #[allow(dead_code)]
    pub fn money(&self) -> u64 {
        self.inventory.balance()
    }

    pub fn id(&self) -> u64 {
        self.id
    }

    pub fn add_item(&mut self, item_id: usize, quantity: u64) -> Kb2Result<()>{
        self.inventory.add_item(item_id, quantity);
        self.save()?;
        Ok(())
    }

    #[allow(dead_code)]
    pub fn remove_item(&mut self, item_id: usize, quantity: u64) -> Kb2Result<()> {
        self.inventory.remove_item(item_id, quantity);
        self.save()?;
        Ok(())
    }

    pub fn save(&mut self) -> Kb2Result<()> {
        let inv_str = self.inventory.dump();
        
        self.connector.update_player_inventory(self.id, inv_str)?;

        Ok(())
    }

    pub fn max_can_craft(&self, r: &Recipe) -> u64 {
        self.inventory.how_many_recipe(r)
    }

    fn get_exploit_quantity(&self, gm: &GameData) -> u64 {
        let mut rng = rand::thread_rng();
        let pcng: f32 = rng.gen();

        ((pcng + 0.1)/(1.0) * self.get_total_multiplier(gm) * BASE_QUANTITY).ceil() as u64
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

    pub fn exploit(&mut self, gamedata: &GameData) -> Kb2Result<Vec<(Item, u64)>> {

        self.energy_use();

        if self.inventory.item_quantity(special_item::ENERGY) == 0 {
            Err(ErrorType::from("No energy left"))
        } else {

            let mut res = Vec::new();

            res.push(
                (
                    self.select_exploit_item(gamedata).unwrap(), // this shouldn't panic unless the object table is modified
                    self.get_exploit_quantity(gamedata)
                )
            );

            self.remove_item(special_item::ENERGY, 1)?;

            for (el, q) in &res {
                self.add_item(el.id(), *q)?;
            }

            Ok(res)
        }
    }

    pub fn craft(&mut self, gamedata: &GameData, recipe_id: usize, quantity: u64) -> Kb2Result<Recipe> {
        let rcp = quantity * gamedata.get_recipe_by_id(recipe_id).ok_or("Recipe not found".to_owned())?;

        self.inventory.craft(&rcp)?;
        self.save()?;

        Ok(rcp.clone())
    } 

    #[allow(dead_code)]
    pub fn clear_inventory(&mut self) -> Kb2Result<()> {
        self.inventory.clear();
        self.save()?;
        Ok(())
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
        format!(
            "{}\r\n{}\r\n{}\r\n{}\r\n{}", 
            self.id, self.inventory.dump_regular(), 
            self.inventory.balance(), 
            self.inventory.dump_position(),
            self.inventory.dump_energy()
        )
    }
}
impl Summary for User {
    fn text(&self) -> String {
        format!("{}\r\n{}", "get_user_", self.to_string())
    }
}