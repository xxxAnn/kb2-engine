use std::{collections::HashMap};

use crate::utils::parser::parse_item_list;

#[derive(Debug, Clone)]
pub struct Recipe {
    inputs: Vec<(usize, u64)>,
    outputs: Vec<(usize, u64)>
}

#[derive(Debug, Clone)]
pub enum ItemClass {
    Currency,
    Resource,
    Tool
}

#[derive(Debug, Clone)]
pub struct Item {
    id: usize,
    name: String,
    class: ItemClass,
    exploit: f32,
    fishing: f32,
    edible: bool,
    exploitable: u32,
    fishable: u32
}

#[derive(Debug, Clone)]
pub struct GameData {
    recipes: Vec<Recipe>,
    items: HashMap<usize, Item>,
}

impl ItemClass {
    pub fn new(inp: impl Into<String>) -> Self {
        match inp.into().as_ref() {
            "Currency" => Self::Currency,
            "Tool" => Self::Tool,
            _ => Self::Resource
        }
    }
}


impl Item {
    pub fn new(fields: &[&str]) -> Self {
        assert!(fields.len() == 8, "Invalid item in Object Table");
        let id: usize = fields[0].parse().unwrap();
        let name: String = fields[1].to_string().replace('_', " ");
        let class: ItemClass = ItemClass::new(fields[2]);
        let exploit: f32 = fields[3].parse().unwrap();
        let fishing: f32 = fields[4].parse().unwrap();
        let edible: bool = fields[5] != "0";
        let exploitable: u32 = fields[6].parse().unwrap();
        let fishable: u32 = fields[7].parse().unwrap();

        Self {
            id,
            name,
            class,
            exploit,
            fishing,
            edible,
            exploitable,
            fishable
        }
    }

    pub fn id(&self) -> usize {
        self.id
    }

    pub fn exploit(&self) -> f32 {
        self.exploit
    }

    
}

impl ToString for ItemClass {
    fn to_string(&self) -> String {
        match self {
            ItemClass::Currency => "Currency".to_string(),
            ItemClass::Resource => "Resource".to_string(),
            ItemClass::Tool => "Tool".to_string(),
        }
    }
}

impl ToString for Item {
    fn to_string(&self) -> String {
        format!("{},{},{},{},{},{},{}", 
            self.id, 
            self.name.replace(' ', "_"), 
            self.class.to_string(), 
            self.exploit, 
            self.fishing, 
            self.edible, 
            self.fishable)
    }
}

impl Recipe {
    pub fn new(inputs: Vec<(usize, u64)>, outputs: Vec<(usize, u64)>) -> Self {
        Self { inputs, outputs }
    }

    pub fn inps(&self) -> &[(usize, u64)] {
        &self.inputs
    }
}

impl ToString for Recipe {
    fn to_string(&self) -> String {
        let inps = self.inputs.iter().map(|(id, q)| format!("{id}:{q}")).collect::<Vec<String>>().join(",");
        let outs = self.outputs.iter().map(|(id, q)| format!("{id}:{q}")).collect::<Vec<String>>().join(",");
        format!("{inps}->{outs}")
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
        self.items.iter().filter(|(_, v)| v.exploitable != 0).map(|(_, v)| (v, v.exploitable)).collect()
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

                let item = Item::new(&fields);

                res.insert(item.id(), item);
            }
        }

        res
    }
}