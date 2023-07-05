use crate::{Data, Item, game::message::GameMessage, Kb2Result, ErrorType, Recipe};

use super::{Summary, Summarize};

pub struct GetRecipes<'a> {
    data: &'a mut Data,
}

impl<'a> GetRecipes<'a> {
    pub fn new(data: &'a mut Data) -> Self {
        Self {
            data,
        }
    }
}

impl<'a> Summarize<'a> for GetRecipes<'a> {
    type ResultSummary = String;

    fn call(self) -> Kb2Result<String> {
        Ok(format!("get_recipes_\r\n{}", self.data.gamedata().recipes_text()))
    }

    fn from_message(data: &'a mut Data, _gm: &GameMessage) -> Kb2Result<Self> {
        Ok(GetRecipes::new(data))
    }
}

