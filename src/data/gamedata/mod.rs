use std::collections::HashMap;

use crate::{utils::parser::{parse_item_list, extract_item_data}, defs::{OBJECT_TABLE_FILE, CRAFT_RECIPES_FILE}};

mod item;
mod recipe;
mod map;

pub use item::Item;
pub use recipe::Recipe;
pub use map::{MapData, TileType, TileClass};

#[derive(Debug, Clone)]
pub struct GameData {
    recipes: Vec<Recipe>,
    items: HashMap<usize, Item>,
    map: MapData
}

impl GameData {
    pub fn new() -> Self {
        let recipes = Self::generate_recipes();

        let items = Self::generate_items();

        let map = MapData::new();

        Self {
            recipes,
            items,
            map
        }
    }

    pub fn get_multiplier(&self, tt: &TileType, i: usize) -> u64 {
        let cls = self.tile(&tt);

        cls.get_multiplier(i)
    }

    pub fn tile(&self, tt: &TileType) -> &TileClass {
        self.map.tile(tt).unwrap() // shouldn't error probably
    } 

    pub fn get_recipe_by_id(&self, _id: usize) -> Option<&Recipe> {
        self.recipes.get(0)
    }   

    pub fn recipes_text(&self) -> String {
        let count = self.recipes.len();
        let recipes_txt = self.recipes.iter().map(std::string::ToString::to_string).collect::<Vec<String>>().join("\r\n");
        format!("{count}\r\n{recipes_txt}")
    }

    pub fn get_recipes(&self) -> &[Recipe] {
        &self.recipes
    }

    pub fn get_item_by_id(&self, id: usize) -> Option<&Item> {
        self.items.get(&id)
    }

    pub fn get_exploitable(&self) -> Vec<(&Item, u32)> {
        self.items.iter().filter(|(_, v)| v.exploitable() != 0).map(|(_, v)| (v, v.exploitable())).collect()
    }

    fn generate_recipe(s: &str) -> Option<Recipe> {
        let mut inpout = s.split("->");
                
        let inp = inpout.next()?;
        let out = inpout.next()?;

        let inps = parse_item_list(inp)?;
        let outs = parse_item_list(out)?;

        Some(Recipe::new(inps, outs))
    }

    fn generate_recipes() -> Vec<Recipe> {
        let mut res = Vec::new();

        for line in std::fs::read_to_string(CRAFT_RECIPES_FILE).unwrap().lines() { // shouldn't panic unless the file is broken
            if !line.starts_with('#') {
                if let Some(rcp) = Self::generate_recipe(line) {
                    res.push(rcp);
                }
            }
        }

        res
    }

    fn generate_items() -> HashMap<usize, Item> {
        let mut res = HashMap::new();

        for line in std::fs::read_to_string(OBJECT_TABLE_FILE).unwrap().lines() { // shouldn't panic unless the file is broken
            if !line.starts_with('#') {
                let fields: Vec<&str> = line.split(',').collect();

                if let Ok(item) = extract_item_data(&fields) {
                    res.insert(item.id(), item);
                }
            }
        }

        res
    }
}