

use crate::{prelude::Data, game::message::GameMessage, defs::ErrorType};

use super::{Summary, Summarize};

pub struct AvailableRecipes<'a> {
    data: &'a mut Data,
    userid: u64
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

impl Summary for AvailableRecipesSummary {
    fn text(&self) -> String {
        format!("{}\r\n{}\r\n{}\r\n", "available_recipes_", self.recipe_ids.len(), self.recipe_ids
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<String>>()
            .join(",")
        )
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

impl<'a> Summarize<'a> for AvailableRecipes<'a> {
    type ResultSummary = AvailableRecipesSummary;

    fn call(self) -> Result<AvailableRecipesSummary, ErrorType> {
        let gamedata = self.data.gamedata();
        let user = self.data.player_mut(self.userid);

        let temp = user.possible_recipes(&gamedata);
        let res = temp
            .iter()
            .map(|(id, _)| *id)
            .collect::<Vec<usize>>();

        Ok(AvailableRecipesSummary::new(&res))
    }

    fn from_message(data: &'a mut Data, gm: &GameMessage) -> Result<Self, ErrorType> {
        Ok(AvailableRecipes::new(data, gm.get_numeric_line(1)?))
    }
}

