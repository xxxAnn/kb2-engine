use crate::{prelude::Data, game::message::GameMessage, defs::ErrorType, data::Recipe};

use super::{Summary, Summarize};

pub struct Craft<'a> {
    data: &'a mut Data,
    userid: u64,
    quantity: u64,
    recipe_id: usize   
}

pub struct CraftSummary {
    rcp: Recipe
}

impl<'a> Craft<'a> {
    pub fn new(data: &'a mut Data, userid: u64, quantity: u64, recipe_id: usize) -> Self {
        Self {
            data,
            userid,
            quantity,
            recipe_id
        }
    }
}

impl CraftSummary {
    fn new(rcp: Recipe) -> Self {
        Self {
            rcp
        }
    }
}

impl Summary for CraftSummary {
    fn text(&self) -> String {
        format!("{}\r\n{}\r\n", "craft_", self.rcp.to_string())
    }
}

impl<'a> Summarize<'a> for Craft<'a> {
    type ResultSummary = CraftSummary;

    fn call(self) -> Result<CraftSummary, ErrorType> {
        let gd = &self.data.gamedata();
        let user = self.data.player_mut(self.userid);

        Ok(CraftSummary::new(user.craft(gd, self.recipe_id, self.quantity)?))
    }

    fn from_message(data: &'a mut Data, gm: &GameMessage) -> Result<Self, ErrorType> {
        Ok(Craft::new(
            data, 
            gm.get_numeric_line(1)?, 
            gm.get_numeric_line(2)?, 
            gm.get_numeric_line(3)?
        ))
    }
}
