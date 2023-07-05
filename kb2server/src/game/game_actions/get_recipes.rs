use crate::{Data, game::message::GameMessage, Result};

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

    fn call(self) -> Result<String> {
        Ok(format!("get_recipes_\r\n{}", self.data.gamedata().recipes_text()))
    }

    fn from_message(data: &'a mut Data, _gm: &GameMessage) -> Result<Self> {
        Ok(GetRecipes::new(data))
    }
}

