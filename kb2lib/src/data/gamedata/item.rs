#[derive(Debug, Clone)]
pub enum ItemClass {
    Special,
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


impl ItemClass {
    pub fn new(inp: impl Into<String>) -> Self {
        match inp.into().as_ref() {
            "Special" => Self::Special,
            "Tool" => Self::Tool,
            _ => Self::Resource
        }
    }
}

impl Item {
    pub fn new(id: usize, name: String, class_str: String, exploit: f32, fishing: f32, edible: bool, exploitable: u32, fishable: u32) -> Self {//fields: &[&str]) -> Self {
        let class = ItemClass::new(class_str);

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

    pub fn exploitable(&self) -> u32 {
        self.exploitable
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
            ItemClass::Special => "Currency".to_string(),
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
