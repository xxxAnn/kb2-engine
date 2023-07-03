use crate::{prelude::Data, defs::ErrorType};

mod exploit;
mod available_recipes;
mod get_user;
mod unknown;
mod get_recipe;
mod get_recipes;
mod craft;

pub use available_recipes::AvailableRecipes;
pub use exploit::Exploit;
pub use get_user::GetUser;
pub use unknown::Unknown;
pub use get_recipe::GetRecipe;
pub use get_recipes::GetRecipes;
pub use craft::Craft;

use super::message::GameMessage;

pub trait Summary {
    fn text(&self) -> String;
}

pub trait Summarize<'a> {
    type ResultSummary: Summary;

    fn call(self) -> Result<Self::ResultSummary, ErrorType>;
    fn from_message(data: &'a mut Data, gm: &GameMessage) -> Result<Self, ErrorType>
    where Self: Sized;
}

impl Summary for String {
    fn text(&self) -> String {
        self.clone()
    }
}