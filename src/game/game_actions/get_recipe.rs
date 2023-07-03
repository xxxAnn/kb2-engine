use rand::Error;

use crate::{prelude::Data, game::message::GameMessage, defs::ErrorType};

use super::{Summary, Summarize};

pub struct GetRecipe<'a> {
    data: &'a mut Data,
    recipe_id: usize
}

impl<'a> GetRecipe<'a> {
    pub fn new(data: &'a mut Data, recipe_id: usize) -> Self {
        Self {
            data,
            recipe_id
        }
    }
}

impl<'a> Summarize<'a> for GetRecipe<'a> {
    type ResultSummary = String;

    fn call(self) -> Result<String, ErrorType> {
        Ok(format!(
            "get_recipe_\r\n{}", 
            self
                .data
                .gamedata()
                .get_recipe_by_id(self.recipe_id)
                .ok_or("Recipe ID not found"
                    .to_owned()
                )?
                .to_string()
            )
        )
    }

    fn from_message(data: &'a mut Data, gm: &GameMessage) -> Result<Self, ErrorType> {
        Ok(GetRecipe::new(data, gm.get_numeric_line(1)?))
    }
}

