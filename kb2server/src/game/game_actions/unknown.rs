use crate::{Data, game::message::GameMessage, Result};

use super::Summarize;

pub struct Unknown;

impl Unknown {
    pub fn new() -> Self {
        Self {}
    }
}

impl Summarize<'_> for Unknown {
    type ResultSummary = String;

    fn call(self) -> Result<String> {
        Ok("Unkown Action Code".to_owned())
    }

    fn from_message(_: &'_ mut Data, _: &GameMessage) -> Result<Self> {
        Ok(Unknown::new())
    }
}

