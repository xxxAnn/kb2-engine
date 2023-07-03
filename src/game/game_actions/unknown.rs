use crate::{prelude::Data, game::message::GameMessage, defs::ErrorType};

use super::Summarize;

pub struct Unknown;

impl Unknown {
    pub fn new() -> Self {
        Self {}
    }
}

impl Summarize<'_> for Unknown{
    type ResultSummary = String;

    fn call(self) -> Result<String, ErrorType> {
        Ok("Unkown Action Code".to_owned())
    }

    fn from_message(_: &'_ mut Data, _: &GameMessage) -> Result<Self, ErrorType> {
        Ok(Unknown::new())
    }
}

