use crate::{prelude::Data, game::message::GameMessage, defs::{Kb2Result}};

use super::{Summarize};

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

