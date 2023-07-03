use crate::{prelude::Data, game::message::GameMessage, defs::ErrorType};

use super::Summarize;

pub struct Unknown<'a> {
    data: &'a mut Data,
}

impl<'a> Unknown<'a> {
    pub fn new(data: &'a mut Data) -> Self {
        Self {
            data
        }
    }
}

impl<'a> Summarize<'a> for Unknown<'a> {
    type ResultSummary = String;

    fn call(self) -> Result<String, ErrorType> {
        Ok("Unkown Action Code".to_owned())
    }

    fn from_message(data: &'a mut Data, gm: &GameMessage) -> Result<Self, ErrorType> {
        Ok(Unknown::new(data))
    }
}

