use crate::prelude::{Data, Item};

pub trait Summary {
    fn text(&self) -> String;
}

pub trait Summarize {
    type ResultSummary: Summary;

    fn call(self) -> Self::ResultSummary;
}

pub struct Exploit<'a> {
    data: &'a mut Data,
    userid: u64   
}

pub struct AvailableRecipes<'a> {
    data: &'a mut Data,
    userid: u64
}

pub struct ExploitSummary {
    item_obtained: Item,
    quantity_obtained: u64
}

pub struct AvailableRecipesSummary {
    recipe_ids: Vec<usize>
}

impl AvailableRecipesSummary {
    fn new(recipe_ids: &[usize]) -> Self {
        Self {
            recipe_ids: recipe_ids.to_owned()
        }
    }
}

impl ExploitSummary {
    fn new(item_obtained: Item, quantity_obtained: u64) -> ExploitSummary {
        Self { 
            item_obtained,
            quantity_obtained
        }
    }
}

impl Summary for AvailableRecipesSummary {
    fn text(&self) -> String {
        format!("{}\r\n{}\r\n", "available_recipes_", self.recipe_ids
            .iter()
            .map(|id| id.to_string())
            .collect::<Vec<String>>()
            .join(",")
        )
    }
}

impl Summary for ExploitSummary {
    fn text(&self) -> String {
        format!("{}\r\n{}\r\n{}", "exploit_", self.quantity_obtained, self.item_obtained.to_string())
    }
}

impl<'a> Exploit<'a> {
    pub fn new(data: &'a mut Data, userid: u64) -> Self {
        Self {
            data,
            userid
        }
    }
}

impl<'a> AvailableRecipes<'a> {
    pub fn new(data: &'a mut Data, userid: u64) -> Self {
        Self {
            data,
            userid
        }
    }
}

impl Summarize for Exploit<'_> {
    type ResultSummary = ExploitSummary;

    fn call(self) -> ExploitSummary {
        let gamedata = self.data.gamedata();
        let user = self.data.player_mut(self.userid);

        let temp = user.exploit(&gamedata);
        let res = temp
            .first()
            .unwrap();

        ExploitSummary::new(res.0.clone(), res.1)
    }
}

impl Summarize for AvailableRecipes<'_> {
    type ResultSummary = AvailableRecipesSummary;

    fn call(self) -> AvailableRecipesSummary {
        let gamedata = self.data.gamedata();
        let user = self.data.player_mut(self.userid);

        let temp = user.possible_recipes(&gamedata);
        let res = temp
            .iter()
            .map(|(id, _)| *id)
            .collect::<Vec<usize>>();

        AvailableRecipesSummary::new(&res)
    }
}