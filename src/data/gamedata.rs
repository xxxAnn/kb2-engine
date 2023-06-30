use std::{collections::HashMap};

use crate::parser::parse_item_list;

#[derive(Debug)]
pub struct Recipe {
    inputs: Vec<(usize, u64)>,
    outputs: Vec<(usize, u64)>
}

#[derive(Debug)]
pub enum ItemClass {
    Currency,
    Resource
}

#[derive(Debug)]
pub struct Item {
    id: usize,
    name: String,
    class: ItemClass,
    exploit: f32,
    fishing: f32,
    edible: bool
}

#[derive(Debug)]
pub struct GameData {
    recipes: Vec<Recipe>,
    items: HashMap<usize, Item>,
}

impl ItemClass {
    pub fn new(inp: impl Into<String>) -> Self {
        match inp.into().as_ref() {
            "Currency" => Self::Currency,
            _ => Self::Resource
        }
    }
}

impl Item {
    pub fn new(fields: Vec<&str>) -> Self {
        if fields.len() != 6 {
            panic!("Invalid item in Object Table");
        }
        let id: usize = fields[0].parse().unwrap();
        let name: String = fields[1].to_string();
        let class: ItemClass = ItemClass::new(fields[2]);
        let exploit: f32 = fields[3].parse().unwrap();
        let fishing: f32 = fields[4].parse().unwrap();
        let edible: bool = fields[5] != "0";

        Self {
            id,
            name,
            class,
            exploit,
            fishing,
            edible,
        }
    }

    pub fn id(&self) -> usize {
        self.id
    }
}

impl Recipe {
    pub fn new(inputs: Vec<(usize, u64)>, outputs: Vec<(usize, u64)>) -> Self {
        Self { inputs, outputs }
    }
}

impl GameData {
    pub fn new() -> Self {
        let recipes = Self::generate_recipes();

        let items = Self::generate_items();

        Self {
            recipes,
            items
        }
    }

    fn generate_recipes() -> Vec<Recipe> {
        let mut res = Vec::new();

        for line in std::fs::read_to_string("gamedata/CRAFT_RECIPES.KB2").unwrap().lines() {
            if !line.starts_with('#') {
                let mut inpout = line.split("->");
                
                let inp = inpout.next().unwrap();
                let out = inpout.next().unwrap();

                let inps = parse_item_list(inp);
                let outs = parse_item_list(out);

                res.push(Recipe::new(inps, outs));
            
            }
        }

        res
    }

    fn generate_items() -> HashMap<usize, Item> {
        let mut res = HashMap::new();

        for line in std::fs::read_to_string("gamedata/OBJECT_TABLE.KB2").unwrap().lines() {
            if !line.starts_with('#') {
                let fields: Vec<&str> = line.split(',').collect();

                let item = Item::new(fields);

                res.insert(item.id(), item);
            }
        }

        res
    }
}