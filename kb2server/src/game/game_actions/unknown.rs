use crate::{Data, Item, game::message::GameMessage, Kb2Result, ErrorType, Recipe};

use super::{Summary, Summarize};

pub struct Unknown;

impl Unknown {
    pub fn new() -> Self {
        Self {}
    }
}

impl Summarize<'_> for Unknown {
    type ResultSummary = String;

    fn call(self) -> Kb2Result<String> {
        Ok("Unkown Action Code".to_owned())
    }

    fn from_message(_: &'_ mut Data, _: &GameMessage) -> Kb2Result<Self> {
        Ok(Unknown::new())
    }
}

